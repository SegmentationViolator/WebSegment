// web segment - a personal website used to host some markdown files and my portfolio
// Copyright (C) 2023 Segmentation Violator

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

use yew::classes;
use yew_router::components::Link;

use crate::config;
use crate::Route;

#[yew::function_component(NavigationBar)]
pub fn navigation_bar() -> yew::Html {
    let current_route: Route = yew_router::hooks::use_route().unwrap();
    let navigator = yew_router::hooks::use_navigator().unwrap();

    let pages = Route::DISPLAYABLE
        .iter()
        .map(|route| {
            let mut classes = classes!("nav-link");
            if current_route == *route {
                classes.push("active");
            }

            yew::html! {
                <li>
                    <Link<Route> classes={classes} to={route.clone()}>
                        <small> {route.to_string()} </small>
                    </Link<Route>>
                </li>
            }
        })
        .collect::<yew::Html>();

    yew::html! {
        <div class={yew::classes!("nav-bar")}>
            <h2 onclick={move |_| { navigator.push(&Route::Home); } }  class={yew::classes!("nav-head")}>
                { config::TITLE }
            </h2>
            <ui class={yew::classes!("nav-links")}>
                { pages }
            </ui>
        </div>
    }
}
