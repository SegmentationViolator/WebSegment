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

use yew_markdown::Markdown;
use yew_router::components::Link;
use yew_router::components::Redirect;
use yew_router::Routable;

use crate::title::Title;
use crate::utils;
use crate::Route;

struct PostView {
    body: Option<String>,
    fetch_state: utils::FetchState,
    filename: String,
}

#[derive(PartialEq, yew::Properties)]
struct Props {
    pub filename: String,
}

impl yew::Component for PostView {
    type Message = utils::Message<String, String>;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            body: None,
            fetch_state: utils::FetchState::Pending,
            filename: ctx.props().filename.clone(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                let filename = ctx.props().filename.clone();

                ctx.link().send_future(async move {
                    let base = web_sys::window().unwrap().location().origin().unwrap();

                    let post = match reqwest::get(format!("{base}/files/{filename}"))
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
                            Ok(text) => text,
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
            utils::Message::UpdateData(filename) => {
                self.filename = filename;
                false
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.fetch_state {
            utils::FetchState::Complete => {
                let filename = ctx.props().filename.clone();

                if self.filename != filename {
                    let link = ctx.link();
                    link.send_message(utils::Message::UpdateData(filename));
                    link.send_message(utils::Message::SetState(utils::FetchState::Pending));
                    return yew::html!(<></>)
                }

                let body = self
                    .body
                    .clone()
                    .expect("body shouldn't be None while fetch_state is Complete");

                let mut components = yew_markdown::CustomComponents::new();

                components.register("UseTitle", |props| {
                    let title: String = props.get_parsed("title")?;
                    Ok(yew::html!( <Title title={title} /> ))
                });
                components.register("UseLink", |props| {
                    let link: String = props.get_parsed("link")?;
                    let text: String = props.get_parsed("text")?;
                    let Some(route) = Route::recognize(&link) else {
                        return Err("invalid path".into());
                    };

                    Ok(yew::html!( <Link<Route> to={route}>{text}</Link<Route>> ))
                });

                yew::html! {
                    <div class="post">
                        <Markdown src={body} components={components}/>
                    </div>
                }
            }
            utils::FetchState::NotFound => yew::html!( <Redirect<Route> to={Route::NotFound} /> ),
            utils::FetchState::Error(error_message) => {
                yew::html!( <p class={yew::classes!("status", "error")}>{error_message}</p> )
            }
            utils::FetchState::Ongoing => {
                yew::html!( <p class={yew::classes!("status")}>{"Fetching..."}</p> )
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                yew::html!(<></>)
            }
        }
    }
}

pub fn post(filename: String) -> yew::Html {
    yew::html!(<PostView filename={filename}/>)
}
