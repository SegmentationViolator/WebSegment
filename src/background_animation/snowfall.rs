use std::cell::OnceCell;
use std::cell::RefCell;
use std::f64;
use std::iter;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::js_sys::Date;
use web_sys::js_sys::Math::random;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

const BASE_DISPLACEMENT_X: f64 = MAX_RADIUS * 0.001;
const BASE_DISPLACEMENT_Y: f64 = MAX_RADIUS * 2.0;
const BASE_SNOWFALL_PROBABILITY: f64 = 0.05;
const MAX_DENSITY: f64 = 100.0;
const MAX_RADIUS: f64 = 5.0;
const MAX_SNOWFLAKES: usize = 256;
const MAX_SWAY_FREQUENCY: f64 = 1.0 / 5.0; // once per 5 ticks

type Data = Rc<RefCell<Vec<Snowflake>>>;

#[derive(Debug)]
struct Snowflake {
    density: f64,
    radius: f64,
    sway_frequency: f64,
    x: f64,
    y: f64,
}

fn animate(context: &Rc<super::Context>, data: &Data) {
    let window = web_sys::window().unwrap_throw();

    let canvas = &context.canvas;

    let height = window.inner_height().unwrap_throw().as_f64().unwrap_throw();
    let width = window.inner_width().unwrap_throw().as_f64().unwrap_throw();

    let renderer = &context.renderer;

    let time = (Date::now() - context.timestamp) / 1000.0;

    canvas.set_height(height as u32);
    canvas.set_width(width as u32);

    renderer.clear_rect(0.0, 0.0, width, height);

    let mut snowflakes = data.borrow_mut();
    let snowfall_propability =
        (snowflakes.len() as f64 / (width / 4.0).min(MAX_SNOWFLAKES as f64)) * BASE_SNOWFALL_PROBABILITY;

    let snowfall_occured = random() < snowfall_propability;

    for snowflake in snowflakes.iter_mut() {
        let relative_density = snowflake.density / MAX_DENSITY;
        let surface_area = f64::consts::PI * snowflake.radius;

        renderer.begin_path();
        renderer
            .arc(
                snowflake.x,
                snowflake.y,
                snowflake.radius,
                0.0,
                2.0 * f64::consts::PI,
            )
            .unwrap_throw();
        renderer.set_fill_style(&JsValue::from_str(&format!(
            "rgb(212 212 212 / {})",
            relative_density,
        )));
        renderer.fill();

        snowflake.x = snowflake.x
            + BASE_DISPLACEMENT_X * surface_area / relative_density
                * (time * snowflake.sway_frequency).sin();

        if snowflake.x > width {
            snowflake.x = snowflake.x % width + random() * width/2.0;
            snowflake.y = random() * height * 0.1;
            continue;
        }

        snowflake.y = snowflake.y + BASE_DISPLACEMENT_Y * relative_density / surface_area;

        if snowflake.y > height {
            snowflake.x = (random() * width + MAX_RADIUS) % width;
            snowflake.y = snowflake.y % height;
        }
    }

    if snowfall_occured && snowflakes.len() < (width / 2.0).min(MAX_SNOWFLAKES as f64) as usize {
        snowflakes.push(Snowflake {
            density: (random() * MAX_DENSITY).max(MAX_DENSITY * 0.6),
            radius: (random() * MAX_RADIUS).max(MAX_RADIUS * 0.8),
            sway_frequency: 2.0 * f64::consts::PI * (random() * MAX_SWAY_FREQUENCY),
            x: (random() * width + MAX_RADIUS) % width,
            y: random() * height * 0.1,
        })
    }

    let closure_ref = context
        .closure
        .get()
        .expect("Guaranteed to be initiated")
        .as_ref();

    web_sys::window()
        .unwrap()
        .request_animation_frame(closure_ref.unchecked_ref())
        .unwrap();
}

pub fn initiate(target_id: String) {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();

    let height = window.inner_height().unwrap_throw().as_f64().unwrap_throw();
    let width = window.inner_width().unwrap_throw().as_f64().unwrap_throw();

    let canvas: HtmlCanvasElement = document
        .get_element_by_id(&target_id)
        .unwrap_throw()
        .dyn_into()
        .expect_throw("Explicitly created a canvas element");

    canvas.set_height(height as u32);
    canvas.set_width(width as u32);

    let renderer: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into()
        .unwrap_throw();

    let closure = Rc::new(OnceCell::new());

    let mut data = Vec::with_capacity((width as usize / 4).min(MAX_SNOWFLAKES));
    data.extend(
        iter::repeat_with(|| Snowflake {
            density: (random() * MAX_DENSITY).max(MAX_DENSITY * 0.6),
            radius: (random() * MAX_RADIUS).max(MAX_RADIUS * 0.8),
            sway_frequency: 2.0 * f64::consts::PI * (random() * MAX_SWAY_FREQUENCY),
            x: (random() * width + MAX_RADIUS) % width,
            y: random() * height * 0.4,
        })
        .take(width as usize / 16),
    );

    let context = Rc::new(super::Context {
        canvas,
        closure: closure.clone(),
        renderer,
        timestamp: Date::now(),
    });

    let data = Rc::new(RefCell::new(data));

    let closure_ref = closure
        .get_or_init(|| {
            Closure::new(move || {
                animate(&context, &data);
            })
        })
        .as_ref();

    web_sys::window()
        .unwrap()
        .request_animation_frame(closure_ref.unchecked_ref())
        .unwrap();
}
