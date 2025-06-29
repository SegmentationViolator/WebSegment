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

use serde::Deserialize;

use crate::card::Card;
use crate::title::Title;
use crate::utils;
use crate::Route;

#[derive(Deserialize)]
struct Post {
    title: String,
    date: String,
    filename: String,
}

struct PostList {
    posts: Vec<Post>,
    fetch_state: utils::FetchState,
}

impl yew::Component for PostList {
    type Message = utils::Message<Vec<Post>, utils::Never>;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            posts: Vec::with_capacity(0),
            fetch_state: utils::FetchState::Pending,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                ctx.link().send_future(async move {
                    let base = web_sys::window().unwrap().location().origin().unwrap();

                    let posts = match reqwest::get(format!("{base}/posts.json"))
                        .await
                        .and_then(|response| response.error_for_status())
                    {
                        Err(error) => {
                            if let Some(reqwest::StatusCode::NOT_FOUND) = error.status() {
                                return utils::Message::SetContent(Vec::new());
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
                            Ok(text) => match serde_json::from_str(&text) {
                                Err(error) => {
                                    return utils::Message::SetState(utils::FetchState::Error(
                                        error.to_string(),
                                    ))
                                }
                                Ok(posts) => posts,
                            },
                        },
                    };

                    utils::Message::SetContent(posts)
                });

                self.fetch_state = utils::FetchState::Ongoing;
                true
            }
            utils::Message::SetContent(posts) => {
                self.posts = posts;

                self.fetch_state = utils::FetchState::Complete;
                true
            }
            utils::Message::SetState(state) => {
                self.fetch_state = state;
                true
            }
            _ => unreachable!() // Message::UpdateData is never sent
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.fetch_state {
            utils::FetchState::Complete => {
                if self.posts.is_empty() {
                    return yew::html! {
                        <>
                            <Title title="Posts" />
                            <p>{"Nothing to see here."}</p>
                        </>
                    };
                }

                let cards = self.posts.iter().map(|post| {
                    yew::html!(
                        <Card
                            title={post.title.clone()}
                            url={utils::Url::Internal(Route::Post { filename: post.filename.clone() })}
                            subtext={post.date.clone()}
                        />
                    )
                });

                yew::html! {
                    <>
                        <Title title="Posts" />
                        <div class={yew::classes!("card-grid")}>
                            { for cards }
                        </div>
                    </>
                }
            }
            utils::FetchState::Error(error_message) => {
                yew::html! {
                    <>
                        <Title title="Posts" />
                        <p class={yew::classes!("status", "error")}>{error_message}</p>
                    </>
                }
            }
            utils::FetchState::Ongoing => {
                yew::html! {
                    <>
                        <Title title="Posts" />
                        <p class={yew::classes!("status")}>{"Fetching..."}</p>
                    </>
                }
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                yew::html!( <Title title="Posts" /> )
            }
            _ => unreachable!(), // FetchState::NotFound is never set as fetch_state
        }
    }
}

pub fn posts() -> yew::Html {
    yew::html!(<PostList />)
}
