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
use yewdux::prelude::*;

use crate::card::Card;

use super::post::PostStore;

#[function_component]
pub fn Posts() -> Html {
    let post_store = use_store_value::<PostStore>();

    let cards = post_store.posts.iter().map(|(filename, meta)| {
        html!(
            <Card
                title={meta.title.clone()}
                url={format!("/post/{filename}")}
                subtext={meta.date.clone()}
            />
        )
    });

    if cards.len() == 0 {
       return html!(<p>{"Nothing to see here...yet."}</p>)
    }

    html!(
        <div class={classes!("card-grid")}>
            {cards.collect::<Html>()}
        </div>
    )
}

pub fn posts() -> Html {
    html!(<Posts />)
}
