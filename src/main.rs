// web segment - a personal website used to host some text files and my portfolio
// Copyright (C) 2023  Segmentation Violator

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt;

use yew::prelude::*;
use yew_router::prelude::*;

mod card;
mod config;
mod footer;
mod hi;
mod navigation_bar;
mod pages;

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
    pub const DISPLAYABLE: &'static [Self] = &[Self::Home, Self::Projects, Self::Posts];
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[function_component(App)]
fn app() -> Html {
    let splashed = yew::use_state_eq(|| false);

    if *splashed {
        return yew::html! {
            <>
                <div id="App">
                    <BrowserRouter>
                        <NavigationBar />
                        <div class="body">
                            <Switch<Route> render={switch} />
                        </div>
                        <Footer />
                    </BrowserRouter>
                </div>
            </>
        };
    }

    let timeout = gloo_timers::callback::Timeout::new(800, move || {
        splashed.set(true);
    });
    timeout.forget();

    yew::html! {
        <>
            <div id="Splash">
                <h1 id="Splash-inner">{ config::TITLE }</h1>
            </div>
        </>
    }
}

fn switch(route: Route) -> Html {
    let document = web_sys::window().unwrap().document().unwrap();
    let app = document.get_element_by_id("App").unwrap();

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
