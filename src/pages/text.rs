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

use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub enum FetchState {
    Failed(String),
    Fetching,
    NotFetching,
    NotFound,
    Success(String),
}

pub enum Message {
    FetchData(String),
    SetState(FetchState),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    filename: String,
}

pub struct Text {
    fetch_state: FetchState,
}

impl Component for Text {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            fetch_state: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::FetchData(filename) => {
                ctx.link().send_future(async move {
                    let base = web_sys::window().unwrap().location().origin().unwrap();

                    let texts: Vec<String> = match reqwest::get(format!("{base}/texts/index.list"))
                        .await
                        .and_then(|response| response.error_for_status())
                    {
                        Err(error) => {
                            return Message::SetState(FetchState::Failed(format!(
                                "Couldn't fetch index.list, {error}"
                            )));
                        }
                        Ok(response) => {
                            let json = response
                                .text()
                                .await
                                .map_err(|error| error.to_string())
                                .and_then(|text| {
                                    serde_json::from_str(&text)
                                        .map_err(|_| "Invalid index.list file".to_string())
                                });

                            if json.is_err() {
                                return Message::SetState(FetchState::Failed(json.unwrap_err()));
                            }

                            json.unwrap()
                        }
                    };

                    if !texts.contains(&filename) {
                        return Message::SetState(FetchState::NotFound);
                    }

                    let text = match reqwest::get(format!("{base}/texts/{filename}")).await {
                        Err(error) => {
                            return Message::SetState(FetchState::Failed(error.to_string()));
                        }
                        Ok(response) => {
                            let text = response.text().await;

                            if text.is_err() {
                                return Message::SetState(FetchState::Failed(
                                    text.unwrap_err().to_string(),
                                ));
                            }

                            text.unwrap()
                        }
                    };

                    return Message::SetState(FetchState::Success(text));
                });

                ctx.link()
                    .send_message(Message::SetState(FetchState::Fetching));
                false
            }
            Message::SetState(state) => {
                self.fetch_state = state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.fetch_state {
            FetchState::Failed(error) => {
                html!( <p class={classes!("status", "error")}>{error}</p> )
            }
            FetchState::Fetching => html!( <p class={classes!("status")}>{"Fetching..."}</p> ),
            FetchState::NotFetching => {
                ctx.link()
                    .send_message(Message::FetchData(ctx.props().filename.clone()));
                html!(<></>)
            }
            FetchState::NotFound => return html!( <Redirect<Route> to={Route::NotFound} /> ),
            FetchState::Success(text) => {
                html! {
                    <>
                        <h4>
                            <a style="float: right"
                                href={
                                    format!("/texts/{}", ctx.props().filename)
                                }
                            >{"View Raw"}</a>
                        </h4>
                        <br/>
                        <br/>

                        <p class={classes!("text")}>{text}</p>
                    </>
                }
            }
        }
    }
}

pub fn text(filename: String) -> Html {
    html!(<Text filename={filename}/>)
}
