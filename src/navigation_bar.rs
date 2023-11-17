// web segment - a personal website used to host some text files and my portfolio
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

    let segmented = yew::use_state_eq(|| false);
    let set_segmented = {
        let segmented = segmented.clone();
        Callback::from(move |_| segmented.set(true))
    };

    let pages = if current_route == Route::NotFound {
        html! {
            <li><Link<Route> classes={classes!("nav-link")} to={Route::Home} > {Route::Home} </Link<Route>></li>
        }
    } else {
        Route::ALL
            .iter()
            .filter(|route| &current_route != *route)
            .map(|route| html! { <li><Link<Route> classes={classes!("nav-link")} to={route.clone()} > {route} </Link<Route>></li> })
            .collect::<Html>()
    };

    let mut classes = classes!("nav-bar");
    segmented.then(|| classes.push("segmented"));

    html! {
        <div class={classes}>
            <h2>
                {"Web "}
                <span onclick={set_segmented}>
                    {"Segment"}
                </span>
            </h2>
            <ui class={classes!("nav-links")}>
                { pages }
            </ui>
        </div>
    }
}
