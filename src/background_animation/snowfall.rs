use std::cell::OnceCell;
use std::cell::RefCell;
use std::f64;
use std::iter;
use std::rc::Rc;

use wasm_bindgen::JsValue;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::js_sys::Math::random;

const BASE_DISPLACEMENT: f64 = MAX_RADIUS * 0.5;
const BASE_SNOWFALL_PROBABILITY: f64 = 0.05;
const MAX_DENSITY: f64 = 100.0;
const MAX_RADIUS: f64 = 2.0;
const MAX_SNOWFLAKES: usize = 64;

type Data = Rc<RefCell<Vec<Snowflake>>>;

#[derive(Debug)]
struct Snowflake {
    density: f64,
    radius: f64,
    x: f64,
    y: f64,
}

fn animate(context: &Rc<super::Context>, data: &Data) {
    let canvas = &context.canvas;
    let renderer = &context.renderer;

    let height = canvas.height() as f64;
    let width = canvas.width() as f64;
    renderer.clear_rect(0.0, 0.0, width, height);

    let mut snowflakes = data.borrow_mut();
    let snowfall_propability = (snowflakes.len() as f64 / MAX_SNOWFLAKES as f64) * BASE_SNOWFALL_PROBABILITY;

    let snowfall_occured = random() < snowfall_propability;

    for snowflake in snowflakes.iter_mut() {
        let relative_density = snowflake.density / MAX_DENSITY;
        let surface_area = f64::consts::PI * snowflake.radius;

        renderer.begin_path();
        renderer.arc(snowflake.x, snowflake.y, snowflake.radius, 0.0, 2.0 * f64::consts::PI).unwrap_throw();
        renderer.set_fill_style(&JsValue::from_str(
            &format!(
                "rgb(212 212 212 / {})",
                relative_density,
            )
        ));
        renderer.fill();

        snowflake.y = (snowflake.y + BASE_DISPLACEMENT * relative_density / surface_area) % height;
    }

    if snowfall_occured && snowflakes.len() < MAX_SNOWFLAKES * 2 {
        snowflakes.push(Snowflake {
            density: (random() * MAX_DENSITY).max(MAX_DENSITY * 0.6),
            radius: (random() * MAX_RADIUS).max(MAX_RADIUS * 0.8),
            x: (random() * width * MAX_RADIUS) + MAX_RADIUS,
            y: (random() * height * 0.2),
        })
    }

    let closure_ref = context.closure.get().expect("Guaranteed to be initiated").as_ref();

    web_sys::window().unwrap().request_animation_frame(closure_ref.unchecked_ref()).unwrap();
}

pub fn initiate(target_id: String) {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document
        .get_element_by_id(&target_id)
        .unwrap()
        .dyn_into()
        .expect("Explicitly created a canvas element");

    let renderer: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let closure = Rc::new(OnceCell::new());

    let height = canvas.height() as f64;
    let width = canvas.width() as f64;

    let mut data = Vec::with_capacity(MAX_SNOWFLAKES);
    data.extend(iter::repeat_with(|| Snowflake {
        density: (random() * MAX_DENSITY).max(MAX_DENSITY * 0.6),
        radius: (random() * MAX_RADIUS).max(MAX_RADIUS * 0.8),
        x: (random() * width * MAX_RADIUS) + MAX_RADIUS,
        y: (random() * height * 0.2),
    }).take(MAX_SNOWFLAKES/2));

    let context = Rc::new(super::Context {
        canvas,
        closure: closure.clone(),
        renderer,
    });

    let data = Rc::new(RefCell::new(data));

    let closure_ref = closure.get_or_init(|| Closure::new(move || {
        animate(&context, &data);
    })).as_ref();

    web_sys::window().unwrap().request_animation_frame(closure_ref.unchecked_ref()).unwrap();
}
