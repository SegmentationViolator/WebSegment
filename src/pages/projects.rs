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
// along with this program.  If not, see <https://www.gnu.org/licenses/>

use web_sys::js_sys::Date;

use serde::Deserialize;

use crate::card::Card;
use crate::title::Title;
use crate::{config, utils};

#[derive(Deserialize)]
struct Error {
    detail: String,
}

#[derive(PartialEq, Deserialize)]
struct Project {
    author: String,
    name: String,
}

struct ProjectList {
    projects: Vec<Project>,
    fetch_state: utils::FetchState,
}

impl Project {
    fn to_card(&self) -> yew::Html {
        let full_name = format!("{}/{}", self.author, self.name);

        let image_url = format!(
            "https://opengraph.githubassets.com/{}/{}",
            Date::now() as u64 / (1000 * 60 * 5),
            full_name,
        );
        let url = format!("https://github.com/{}", full_name);

        yew::html!(
            <Card
                title={full_name}
                url={utils::Url::External(url)}
                image_url={image_url}
            />
        )
    }
}

impl yew::Component for ProjectList {
    type Message = utils::Message<Vec<Project>, utils::Never>;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            projects: Vec::with_capacity(0),
            fetch_state: utils::FetchState::Pending,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                ctx.link().send_future(async {
                    match reqwest::get(format!(
                        "https://pinned.berrysauce.dev/get/{}",
                        config::GITHUB_USERNAME
                    ))
                    .await
                    {
                        Err(error) => {
                            utils::Message::SetState(utils::FetchState::Error(
                                error.to_string(),
                            ))
                        }
                        Ok(response) => {
                            if response.status() != 200 {
                                match response.json::<Error>().await {
                                    Err(error) => {
                                        return utils::Message::SetState(utils::FetchState::Error(
                                            error.to_string(),
                                        ));
                                    }
                                    Ok(error) => {
                                        return utils::Message::SetState(utils::FetchState::Error(
                                            error.detail,
                                        ));
                                    }
                                }
                            }

                            match response.json().await {
                                Err(error) => {
                                    utils::Message::SetState(utils::FetchState::Error(
                                        error.to_string(),
                                    ))
                                }
                                Ok(projects) => utils::Message::SetContent(projects),
                            }
                        }
                    }
                });

                ctx.link()
                    .send_message(utils::Message::SetState(utils::FetchState::Ongoing));
                true
            }
            utils::Message::SetContent(projects) => {
                self.projects = projects;

                self.fetch_state = utils::FetchState::Complete;
                true
            }
            utils::Message::SetState(state) => {
                self.fetch_state = state;
                true
            }
            _ => unreachable!(), // Message::UpdateData is never sent
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.fetch_state {
            utils::FetchState::Complete => {
                if self.projects.is_empty() {
                    return yew::html! {
                        <>
                            <Title title="Projects" />
                            <p>{"Nothing to see here."}</p>
                        </>
                    };
                }

                let cards = self.projects.iter().map(|project| project.to_card());

                yew::html! {
                    <>
                        <Title title="Projects" />
                        <div class={yew::classes!("card-grid")}>
                            { for cards }
                        </div>
                    </>
                }
            }
            utils::FetchState::Error(error_message) => {
                yew::html! {
                    <>
                        <Title title="Projects" />
                        <p class={yew::classes!("status", "error")}>{error_message}</p>
                    </>
                }
            }
            utils::FetchState::Ongoing => {
                yew::html! {
                    <>
                        <Title title="Projects" />
                        <p class={yew::classes!("status")}>{"Fetching..."}</p>
                    </>
                }
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                yew::html!( <Title title="Projects" /> )
            }
            _ => unreachable!(), // FetchState::NotFound is never set as fetch_state
        }
    }
}

pub fn projects() -> yew::Html {
    yew::html!( <ProjectList /> )
}
