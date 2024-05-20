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

use crate::{utils, Route};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub url: utils::Url,
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
    let inner = html! {
        <>
            <div class={classes!("card-head")}>
                <h3>{&properties.title}</h3>
                if let Some(subtext) = &properties.subtext {
                    <small class={classes!("card-subtext")}>{subtext.clone()}</small>
                }
            </div>

            if let Some(image_url) = &properties.image_url {
                <img class={classes!("card-image")} src={image_url.clone()}/>
            }
        </>
    };

    if let utils::Url::External(url) = &properties.url {
        let url = url.clone();
        return html! {
            <div onclick={move |_| { let _ = location.set_href(&url); } } class={classes!("card", "hover-scale")}>
                {inner}
            </div>
        }
    }

    let utils::Url::Internal(route) = &properties.url else { unreachable!() };

    html! {
        <Link<Route> to={route.to_owned()} classes={classes!("card-link")}>
            <div class={classes!("card", "hover-scale")}>
                {inner}
            </div>
        </Link<Route>>
    }
}


