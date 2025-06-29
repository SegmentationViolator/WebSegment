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

#[stylist::yew::styled_component(Home)]
pub fn home() -> yew::Html {
    let break_condition = stylist::yew::use_media_query("(max-width: 508px)");
    let hi_image_url = if stylist::yew::use_media_query("(prefers-color-scheme: dark)") {
        "/assets/hi_dark.gif"
    } else {
        "/assets/hi_light.gif"
    };

    yew::html! {
        <>
            <Title title="Home" />

            <div
                class={css!("width: 100%; height: 25vh; background-position: center; background-repeat: no-repeat;")}
                style={format!("background-image: url({})", hi_image_url)}
            >
            </div>

            <p class={css!("font-size: 1.5rem; hyphens: none; line-break: normal; text-align: center;")}>
                {"I am SegV, "}

                if break_condition {
                    <br/>
                }

                {"a hobbyist programmer."}
            </p>

            <br/>

            <p style="text-align: center;">
                {"I am particularly interested in systems programming, computers graphics and game development. "}
                {"I consider my expertise level to be intermediate and have a lot to learn."}
            </p>

            <br/><br/>
        </>
    }
}

pub fn home() -> yew::Html {
    yew::html!(<Home/>)
}
