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
use yew_router::prelude::*;

use crate::indent::Indent;
use crate::Route;

pub fn home() -> Html {
    html! {
        <>
            <p>{"I am,"}</p><br/>

            <h3>{"Segmentation Violator"}</h3><br/>

            <p>
                <Indent width=4 />
                {"I am a hobbyist programmer, my interests in the field of programming are systems programming, graphics programming and web development. "}
                {"I like making software that I find interesting, and learn by doing"}
            </p><br/>

            <p>
                <Indent width=4 />
                {"The projects I have worked on include Discord bots, an emulator, a programming language and a text editor. I have listed some of my projects "}
                <Link<Route> to={Route::Projects}>{"here"}</Link<Route>>
                {" and links to my profiles on various platforms and my E-mail "}
                <Link<Route> to={Route::Links}>{"here"}</Link<Route>>
            </p>
        </>
    }
}
