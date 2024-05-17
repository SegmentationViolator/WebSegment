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

use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::utils;
use crate::Route;

pub struct Post {
    data: Option<PostData>,
    dispatch: Dispatch<PostStore>,
    fetch_state: utils::FetchState,
}

#[derive(Clone, Default, PartialEq, Store, Deserialize, Serialize)]
pub struct PostMeta {
    pub date: String,
    pub title: String,
}

pub struct PostData {
    body: String,
    meta: PostMeta,
}

#[derive(Clone, Default, PartialEq, Store, Deserialize, Serialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct PostStore {
    pub posts: collections::HashMap<String, PostMeta>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub filename: String,
}

impl Component for Post {
    type Message = utils::Message<PostData>;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<PostStore>::global();

        Self {
            data: None,
            dispatch,
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
                                let (title, date, text) = {
                                    let mut lines = text.split_inclusive('\n');

                                    let Some(title) = lines.next() else {
                                        return utils::Message::SetState(utils::FetchState::Error(
                                            "Post file is empty.".to_string()
                                        ))
                                    };

                                    let date = lines.next().unwrap_or("Unknown Date");

                                    (
                                        title.trim().to_string(),
                                        date.trim().to_string(),
                                        lines.collect::<String>()
                                    )
                                };
                                let body = markdown::to_html_with_options(&text, &markdown::Options::gfm())
                                    .expect("Without MDX enabled, there should be no errors");

                                PostData {
                                    body,
                                    meta: PostMeta {
                                        date,
                                        title,
                                    }
                                }
                            }
                        },
                    };

                    utils::Message::SetContent(post)
                });

                self.fetch_state = utils::FetchState::Ongoing;
                true
            }
            utils::Message::SetContent(post) => {
                self.dispatch.reduce_mut(|post_store| {
                    post_store.posts.insert(ctx.props().filename.clone(), post.meta.clone());
                });

                let _ = self.data.insert(post);

                self.fetch_state = utils::FetchState::Complete;
                true
            }
            utils::Message::SetState(state) => {
                if matches!(state, utils::FetchState::NotFound) {
                    self.dispatch.reduce_mut(|content_store| {
                        content_store.posts.remove(&ctx.props().filename);
                    });
                }

                self.fetch_state = state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            utils::FetchState::Complete => {
                let content = self.data.as_ref()
                    .expect("Data shouldn't be None while fetch_state is Complete");
                let body = Html::from_html_unchecked(content.body.clone().into());

                html!(
                    <>
                        <div class={classes!("post")}>
                            <h1>{content.meta.title.clone()}</h1>
                            <small>{content.meta.date.clone()}</small>
                            <br/>
                            <br/>
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
    html!(<Post filename={filename}/>)
}
