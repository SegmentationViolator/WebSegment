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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub url: String,
    #[prop_or_default]
    pub subtext: Option<String>,
    #[prop_or_default]
    pub image_url: Option<String>,
}

#[function_component(Card)]
pub fn card(properties: &Props) -> Html {
    let location = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .location()
        .unwrap();
    let url = properties.url.clone();

    html! {
        <div onclick={move |_| { let _ = location.set_href(&url); } } class={classes!("card", "hover-scale")}>
            <h3 class={classes!("card-title")}>{&properties.title}</h3>
                if let Some(image_url) = &properties.image_url {
                    <img class={classes!("card-image")} src={image_url.clone()}/>
                } else if let Some(subtext) = &properties.subtext {
                    <small class={classes!("card-subtext")}>{subtext.clone()}</small>
                }
        </div>
    }
}
