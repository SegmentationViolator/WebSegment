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

use crate::utils;

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Props {
    pub title: String,
    pub url: utils::Url,
    #[prop_or_default]
    pub subtext: Option<String>,
    #[prop_or_default]
    pub image_url: Option<String>,
}

#[yew::function_component(Card)]
pub fn card(properties: &Props) -> yew::Html {
    let location = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .location()
        .unwrap();
    let navigator = yew_router::hooks::use_navigator().unwrap();

    let inner = yew::html! {
        <>
            <div class={yew::classes!("card-head")}>
                <h3>{&properties.title}</h3>
                if let Some(subtext) = &properties.subtext {
                    <small class={yew::classes!("card-subtext")}>{subtext.clone()}</small>
                }
            </div>

            if let Some(image_url) = &properties.image_url {
                <img class={yew::classes!("card-image")} src={image_url.clone()}/>
            }
        </>
    };

    match &properties.url {
        utils::Url::External(url) => {
            let url = url.clone();

            yew::html! {
                <div onclick={move |_| { let _ = location.set_href(&url); } } class={yew::classes!("card", "hover-scale")}>
                    {inner}
                </div>
            }
        }
        utils::Url::Internal(route) => {
            let route = route.clone();
            yew::html! {
                <div onclick={move |_| { navigator.push(&route); } } class={yew::classes!("card", "hover-scale")}>
                    {inner}
                </div>
            }
        }
    }
}
