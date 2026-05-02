use std::fmt;

use yew_router::{BrowserRouter, Routable, Switch};

mod callout;
mod card;
mod config;
mod footer;
mod navigation_bar;
mod pages;
mod title;

mod utils;

use footer::Footer;
use navigation_bar::NavigationBar;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404.html")]
    NotFound,
    #[at("/post/:filename")]
    Post { filename: String },
    #[at("/posts")]
    Posts,
    #[at("/projects")]
    Projects,
}

impl Route {
    pub const DISPLAYABLE: &'static [Self] = &[Self::Projects, Self::Posts];
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[yew::function_component(App)]
fn app() -> yew::Html {
    return yew::html! {
        <>
            <div id="App">
                <BrowserRouter>
                    <NavigationBar />
                    <div class={yew::classes!("body")}>
                        <Switch<Route> render={switch} />
                    </div>
                    <Footer />
                </BrowserRouter>
            </div>
        </>
    };
}

fn switch(route: Route) -> yew::Html {
    let app = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("App")
        .unwrap();

    match app.class_name().as_str() {
        "fade" => app.set_class_name("fade-again"),
        _ => app.set_class_name("fade"),
    }

    match route {
        Route::Home => pages::home(),
        Route::NotFound => pages::not_found(),
        Route::Post { filename } => pages::post(filename),
        Route::Posts => pages::posts(),
        Route::Projects => pages::projects(),
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
