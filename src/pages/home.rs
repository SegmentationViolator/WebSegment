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

use crate::hi::Hi;

pub fn home() -> Html {
    html! {
        <>
            <Hi />
            <p style="font-size: 1.6rem; text-align: center;">
                {"I am SegV, a hobbyist programmer."}
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
