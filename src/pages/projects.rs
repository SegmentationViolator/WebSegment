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
// along with this program.  If not, see <https://www.gnu.org/licenses/>

use yew::prelude::*;

use crate::card::{self, Card};

const URLS: &[&str] = &[
    "https://github.com/SegmentationViolator/Ruschip",
    "https://github.com/SegmentationViolator/BatCon",
    "https://github.com/SegmentationViolator/WebSegment",
];

pub enum FetchState {
    Failed(String),
    Fetching,
    NotFetching,
    Success(Vec<card::Props>),
}

pub enum Message {
    FetchData,
    SetState(FetchState),
}

pub struct Projects {
    fetch_state: FetchState,
}

impl Component for Projects {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            fetch_state: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::FetchData => {
                ctx.link().send_future(async {
                    let client = match reqwest::Client::builder().build() {
                        Err(error) => {
                            return Message::SetState(FetchState::Failed(
                                error.without_url().to_string(),
                            ))
                        }
                        Ok(client) => client,
                    };

                    let mut cards: Vec<card::Props> = Vec::with_capacity(URLS.len());

                    let responses = futures::join!(
                        fetch_response(&client, format!("https://corsproxy.io/?{}", URLS[0])),
                        fetch_response(&client, format!("https://corsproxy.io/?{}", URLS[1])),
                        fetch_response(&client, format!("https://corsproxy.io/?{}", URLS[2])),
                    );

                    let responses: [_; URLS.len()] = [
                        responses.2,
                        responses.1,
                        responses.0,
                    ];

                    for (url, response) in URLS.iter().zip(responses) {
                        let Ok(response) = response else {
                            return Message::SetState(FetchState::Failed(response.unwrap_err().to_string()));
                        };

                        let dom = match tl::parse(&response, tl::ParserOptions::default()) {
                            Err(error) => {
                                return Message::SetState(FetchState::Failed(
                                    error.to_string(),
                                ))
                            }
                            Ok(dom) => dom,
                        };

                        let Some(meta_tags) = dom.query_selector("meta") else {
                            return Message::SetState(FetchState::Failed(
                                "Couldn't extract meta-data".to_string()
                            ))
                        };
                        let parser = dom.parser();

                        let og_properties: Vec<(&str, Option<&str>)> = meta_tags.filter_map(
                            |tag| tag.get(parser)
                                .and_then(|node| node.as_tag())
                                .and_then(|tag| Some(tag.attributes()))
                                .and_then(|attrs| {
                                    let Some(property) = attrs.get("property").flatten().and_then(|property| property.try_as_utf8_str()) else {
                                        return None;
                                    };

                                    if !property.starts_with("og:") { return None };

                                    attrs.get("content")
                                        .flatten()
                                        .and_then(|content| (property, content.try_as_utf8_str()).into())
                                })
                        ).collect();

                        let Some(image_url) = og_properties.iter().copied().find_map(|(property, content)| {
                            if property == "og:image" { content } else { None }
                        }) else {
                            return Message::SetState(FetchState::Failed("Couldn't extract an image".to_string()))
                        };

                        let url = og_properties.iter().copied().find_map(|(property, content)| {
                            if property == "og:url" { content } else { None }
                        }).unwrap_or(url);

                        let title = og_properties.iter().copied().find_map(|(property, content)| {
                            if property != "og:title" {
                                return None
                            }

                            content.and_then(
                                |text| text.split('/')
                                     .last()
                                     .and_then(|text| text.split(':').next())
                            )
                        }).unwrap_or(url.split('/').last().unwrap());

                        cards.push(card::Props {
                            title: title.to_string(), url: url.to_string(), image_url: image_url.to_string()
                        })
                    }

                    return Message::SetState(FetchState::Success(cards));
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
                ctx.link().send_message(Message::FetchData);
                html!(<></>)
            }
            FetchState::Success(cards) => {
                let cards = cards.iter().map(|card| html!{
                    <Card title={card.title.clone()} url={card.url.clone()} image_url={card.image_url.clone()} />
                });

                html! {
                    <div class={classes!("card-grid")}>
                        { for cards }
                    </div>
                }
            }
        }
    }
}

async fn fetch_response(client: &reqwest::Client, url: String) -> Result<String, reqwest::Error> {
    let response = client.get(url).send().await
        .and_then(|response| response.error_for_status())?;

    Ok(response.text().await?)
}

pub fn projects() -> Html {
    html!( <Projects /> )
}
