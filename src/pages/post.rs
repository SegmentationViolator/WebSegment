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

use crate::utils;
use crate::Route;

pub struct PostView {
    body: Option<String>,
    fetch_state: utils::FetchState,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub filename: String,
}

impl Component for PostView {
    type Message = utils::Message<String>;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            body: None,
            fetch_state: utils::FetchState::Pending,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                let filename = ctx.props().filename.clone();

                ctx.link().send_future(async move {
                    let base = web_sys::window().unwrap().location().origin().unwrap();

                    let post = match reqwest::get(format!("{base}/texts/{filename}"))
                        .await
                        .and_then(|response| response.error_for_status())
                    {
                        Err(error) => {
                            if let Some(reqwest::StatusCode::NOT_FOUND) = error.status() {
                                return utils::Message::SetState(utils::FetchState::NotFound);
                            }

                            return utils::Message::SetState(utils::FetchState::Error(
                                error.to_string(),
                            ));
                        }
                        Ok(response) => match response.text().await {
                            Err(error) => {
                                return utils::Message::SetState(utils::FetchState::Error(
                                    error.to_string(),
                                ))
                            }
                            Ok(text) => {
                                markdown::to_html_with_options(
                                    &text,
                                    &markdown::Options {
                                        compile: markdown::CompileOptions {
                                            allow_dangerous_html: true,
                                            allow_dangerous_protocol: false,
                                            ..markdown::CompileOptions::gfm()
                                        },
                                        parse: markdown::ParseOptions {
                                            constructs: markdown::Constructs {
                                                math_flow: false,
                                                math_text: false,
                                                ..markdown::Constructs::gfm()
                                            },
                                            ..markdown::ParseOptions::gfm()
                                        }
                                    },
                                )
                                .expect("Without MDX enabled, there should be no errors")
                            }
                        },
                    };

                    utils::Message::SetContent(post)
                });

                self.fetch_state = utils::FetchState::Ongoing;
                true
            }
            utils::Message::SetContent(post) => {
                let _ = self.body.insert(post);

                self.fetch_state = utils::FetchState::Complete;
                true
            }
            utils::Message::SetState(state) => {
                self.fetch_state = state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            utils::FetchState::Complete => {
                let html = self
                    .body
                    .as_ref()
                    .expect("Data shouldn't be None while fetch_state is Complete");
                let body = Html::from_html_unchecked(html.clone().into());

                html!(
                    <>
                        <div class={classes!("post")}>
                            {body}
                        </div>
                    </>
                )
            }
            utils::FetchState::NotFound => html!( <Redirect<Route> to={Route::NotFound} /> ),
            utils::FetchState::Error(error_message) => {
                html!( <p class={classes!("status", "error")}>{error_message}</p> )
            }
            utils::FetchState::Ongoing => {
                html!( <p class={classes!("status")}>{"Fetching..."}</p> )
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                html!()
            }
        }
    }
}

pub fn post(filename: String) -> Html {
    html!(<PostView filename={filename}/>)
}
