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

use yew::prelude::*;

use crate::config::{GITHUB_USERNAME, REPOSITORY_NAME};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class={classes!("footer")}>
            <p>
                {"Made with "}
                <a href="https://yew.rs/">{"Yew"}</a>
                {" & "}
                <svg
                    class={classes!("hover-tilt")}
                    height="1em" width="1em"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 512 512"
                >
                    <path d="M47.6 300.4L228.3 469.1c7.5 7 17.4 10.9 27.7 10.9s20.2-3.9 27.7-10.9L464.4 300.4c30.4-28.3 47.6-68 47.6-109.5v-5.8c0-69.9-50.5-129.5-119.4-141C347 36.5 300.6 51.4 268 84L256 96 244 84c-32.6-32.6-79-47.5-124.6-39.9C50.5 55.6 0 115.2 0 185.1v5.8c0 41.5 17.2 81.2 47.6 109.5z"/>
                </svg>
            </p>
            <a href={format!("https://github.com/{GITHUB_USERNAME}/{REPOSITORY_NAME}")}>{"Source Code"}</a>
        </div>
    }
}
