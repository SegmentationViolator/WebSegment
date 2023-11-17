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

const TEXTS: &[(&str, &str)] = include!("../../texts/index");

use yew::prelude::*;
use yew_router::prelude::*;

use crate::paragraph::Paragraph;
use crate::Route;

pub fn txt(name: &str) -> Html {
    let Some(text) = TEXTS.iter().copied().find_map(|(filename, text)|
        if filename == name { Some(text) } else { None }
    ) else {
        return html!(<Redirect<Route> to={Route::NotFound} />)
    };

    html! {
        <Paragraph><pre>{text}</pre></Paragraph>
    }
}
