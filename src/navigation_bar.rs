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

use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {
    let current_route: Route = use_route().unwrap();

    let pages = if matches!(current_route, Route::NotFound | Route::MarkdownFile { .. }) {
        html! {
            <li><Link<Route> classes={classes!("nav-link")} to={Route::Home} > {Route::Home.to_string()} </Link<Route>></li>
        }
    } else {
        Route::DISPLAYABLE
            .iter()
            .filter(|route| &current_route != *route)
            .map(|route| html! { <li><Link<Route> classes={classes!("nav-link")} to={route.clone()} > {route.to_string()} </Link<Route>></li> })
            .collect::<Html>()
    };

    html! {
        <div class={classes!("nav-bar")}>
            <h2>
                {"Web Segment"}
            </h2>
            <ui class={classes!("nav-links")}>
                { pages }
            </ui>
        </div>
    }
}
