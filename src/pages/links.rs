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

use crate::config::{EMAIL, GITHUB_USERNAME};

pub fn links() -> Html {
    html! {
        <ul>
            <li>
                <p>
                    <a href={format!("https://github.com/{GITHUB_USERNAME}/")}>{GITHUB_USERNAME}</a>
                    <sup>{"[GitHub]"}</sup>
                </p>
            </li>
            <li>
                <p>
                    {EMAIL}
                    <sup>{"[E-mail]"}</sup>
                </p>
            </li>
        </ul>
    }
}
