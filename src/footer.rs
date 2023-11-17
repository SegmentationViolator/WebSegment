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

use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <p>
                <a href="https://www.rust-lang.org/">{"Made with the Crab Language"} <img src="/crab.webp"/></a>
                <sup><Link<Route> to={Route::Projects} >{"[source]"}</Link<Route>></sup>
            </p>
        </div>
    }
}
