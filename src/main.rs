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
mod footer;
mod navigation_bar;
mod pages;

mod utils;

use footer::Footer;
use navigation_bar::NavigationBar;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/links")]
    Links,
    #[not_found]
    #[at("/404.html")]
    NotFound,
    #[at("/projects")]
    Projects,
    #[at("/f/:filename")]
    MarkdownFile { filename: String },
}

impl Route {
    pub const DISPLAYABLE: [Self; 3] = [Self::Home, Self::Links, Self::Projects];
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
            <div id="App">
                <BrowserRouter>
                    <NavigationBar />
                    <div class="body">
                        <Switch<Route> render={switch} />
                    </div>
                    <Footer />
                </BrowserRouter>
            </div>
        };
    }

    let timeout = gloo_timers::callback::Timeout::new(800, move || {
        splashed.set(true);
    });
    timeout.forget();

    yew::html! {
        <div id="splash">
            <img src="/logo.svg" id="splash-inner"/>
        </div>
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
        Route::Links => pages::links(),
        Route::NotFound => pages::not_found(),
        Route::Projects => pages::projects(),
        Route::MarkdownFile { filename } => pages::markdown_file(filename),
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
