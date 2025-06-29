// web segment - a personal website used to host some markdown files and my portfolio
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

use crate::title::Title;

pub fn not_found() -> yew::Html {
    yew::html! {
        <>
            <Title title="Not Found" />

            <div
                class={stylist::css!(
                    "align-items: center; display: flex; flex-direction: column; height: 100%; justify-content: center;"
                )}
            >
                <h1 class={stylist::css!("font-size: 10em;")}>{"404"}</h1>
                <h2>{"Page Not Found"}</h2>
            </div>
        </>
    }
}
