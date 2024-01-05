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

use std::collections;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::utils;
use crate::Route;

pub struct MarkdownFile {
    dispatch: Dispatch<MarkdownStore>,
    fetch_state: utils::FetchState,
}

#[derive(Clone, Default, PartialEq, Store)]
pub struct MarkdownStore {
    map: collections::HashMap<String, String>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub filename: String,
}

impl Component for MarkdownFile {
    type Message = utils::Message<String>;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<MarkdownStore>::new();
        let content_store = dispatch.get();

        let fetch_state = if content_store.map.contains_key(&ctx.props().filename) {
            utils::FetchState::Complete
        } else {
            utils::FetchState::Pending
        };

        Self {
            dispatch,
            fetch_state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                let filename = ctx.props().filename.clone();

                ctx.link().send_future(async move {
                    let base = web_sys::window().unwrap().location().origin().unwrap();

                    let text = match reqwest::get(format!("{base}/texts/{filename}"))
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
                                markdown::to_html_with_options(&text, &markdown::Options::gfm())
                                    .expect("Without MDX enabled, there should be no errors")
                            }
                        },
                    };

                    utils::Message::SetContent(text)
                });

                self.fetch_state = utils::FetchState::Ongoing;
                true
            }
            utils::Message::SetContent(content) => {
                let filename = ctx.props().filename.clone();

                self.dispatch.reduce_mut(|content_store| {
                    content_store.map.insert(filename, content);
                });

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
                let content_store = self.dispatch.get();

                let content = content_store
                    .map
                    .get(&ctx.props().filename)
                    .unwrap()
                    .clone();
                let html = Html::from_html_unchecked(content.into());
                html!(
                    <div class={classes!("markdown")}>{html}</div>
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
pub fn markdown_file(filename: String) -> Html {
    html!(<MarkdownFile filename={filename}/>)
}
