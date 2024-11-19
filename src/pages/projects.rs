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

#[derive(PartialEq, Deserialize)]
struct Project {
    full_name: String,
    html_url: String,
    name: String,
}

struct Projects {
    dispatch: yewdux::Dispatch<ProjectStore>,
    fetch_state: utils::FetchState,
}

#[derive(Default, PartialEq, yewdux::Store)]
struct ProjectStore {
    projects: Vec<Project>,
}

impl Project {
    fn to_card(&self) -> yew::Html {
        let image_url = format!(
            "https://opengraph.githubassets.com/{}/{}",
            Date::now() as u64 / (1000 * 60 * 5),
            self.full_name,
        );
        let title = self.name.clone();
        let url = self.html_url.clone();

        yew::html!(
            <Card
                title={title}
                url={utils::Url::External(url)}
                image_url={image_url}
            />
        )
    }
}

impl yew::Component for Projects {
    type Message = utils::Message<Vec<Project>>;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let dispatch = yewdux::Dispatch::<ProjectStore>::global();
        let project_store = dispatch.get();

        let fetch_state = if project_store.projects.is_empty() {
            utils::FetchState::Pending
        } else {
            utils::FetchState::Complete
        };

        Self {
            dispatch,
            fetch_state,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                ctx.link().send_future(async {
                    let projects: Vec<Project> = match reqwest::get(format!(
                        "https://api.github.com/users/{}/starred",
                        config::GITHUB_USERNAME
                    ))
                    .await
                    .and_then(|response| response.error_for_status())
                    {
                        Err(error) => {
                            return utils::Message::SetState(utils::FetchState::Error(
                                error.to_string(),
                            ));
                        }
                        Ok(response) => {
                            let result = match response.text().await {
                                Err(error) => {
                                    return utils::Message::SetState(utils::FetchState::Error(
                                        error.to_string(),
                                    ))
                                }
                                Ok(text) => serde_json::from_str::<Vec<Project>>(&text),
                            };

                            match result {
                                Err(error) => {
                                    return utils::Message::SetState(utils::FetchState::Error(
                                        error.to_string(),
                                    ))
                                }
                                Ok(projects) => projects
                                    .into_iter()
                                    .filter_map(|project| {
                                        let owner_name =
                                            project.full_name.split('/').next().unwrap();

                                        if owner_name != config::GITHUB_USERNAME {
                                            return None;
                                        }

                                        Some(project)
                                    })
                                    .collect(),
                            }
                        }
                    };

                    utils::Message::SetContent(projects)
                });

                ctx.link()
                    .send_message(utils::Message::SetState(utils::FetchState::Ongoing));
                true
            }
            utils::Message::SetContent(projects) => {
                self.dispatch.set(ProjectStore { projects });

                self.fetch_state = utils::FetchState::Complete;
                true
            }
            utils::Message::SetState(state) => {
                self.fetch_state = state;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.fetch_state {
            utils::FetchState::Complete => {
                let project_store = self.dispatch.get();

                let cards = project_store
                    .projects
                    .iter()
                    .map(|project| project.to_card());

                yew::html! {
                    <>
                        <Title text="Projects" />
                        <div class={yew::classes!("card-grid")}>
                            { for cards }
                        </div>
                    </>
                }
            }
            utils::FetchState::Error(error_message) => {
                yew::html! {
                    <>
                        <Title text="Projects" />
                        <p class={yew::classes!("status", "error")}>{error_message}</p>
                    </>
                }
            }
            utils::FetchState::Ongoing => {
                yew::html! {
                    <>
                        <Title text="Projects" />
                        <p class={yew::classes!("status")}>{"Fetching..."}</p>
                    </>
                }
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                yew::html!(<Title text="Projects" />)
            }
            _ => unreachable!(),
        }
    }
}

pub fn projects() -> yew::Html {
    yew::html!( <Projects /> )
}
