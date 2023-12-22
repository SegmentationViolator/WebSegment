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

use yew::prelude::*;
use yewdux::prelude::*;

use serde::Deserialize;

use crate::card::Card;
use crate::utils;

#[derive(PartialEq, Deserialize)]
pub struct Project {
    pub owner: String,
    pub repository: String,
}

pub struct Projects {
    dispatch: Dispatch<ProjectStore>,
    fetch_state: utils::FetchState,
}

#[derive(Default, PartialEq, Store)]
struct ProjectStore {
    projects: Vec<Project>,
    uuid: uuid::Uuid,
}

impl Project {
    fn to_card(&self, uuid: &uuid::Uuid) -> Html {
        let image_url = format!(
            "https://opengraph.githubassets.com/{}/{}/{}",
            uuid, self.owner, self.repository,
        );
        let title = self.repository.clone();
        let url = format!("https://github.com/{}/{}", self.owner, self.repository);

        html!(
            <Card
                title={title}
                url={url}
                image_url={image_url}
            />
        )
    }
}

impl Component for Projects {
    type Message = utils::Message<Vec<Project>>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<ProjectStore>::new();
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                ctx.link().send_future(async {
                    let base = web_sys::window().unwrap().location().origin().unwrap();

                    let projects = match reqwest::get(format!("{base}/projects.json"))
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
                                Ok(projects) => projects,
                            }
                        }
                    };

                    utils::Message::SetContent(projects)
                });

                ctx.link()
                    .send_message(utils::Message::SetState(utils::FetchState::Ongoing));
                false
            }
            utils::Message::SetContent(projects) => {
                self.dispatch.set(ProjectStore {
                    projects,
                    uuid: uuid::Uuid::new_v4(),
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
                let project_store = self.dispatch.get();

                let cards = project_store
                    .projects
                    .iter()
                    .map(|project| project.to_card(&project_store.uuid));

                html! {
                    <div class={classes!("card-grid")}>
                        { for cards }
                    </div>
                }
            }
            utils::FetchState::Error(error) => {
                html!( <p class={classes!("status", "error")}>{error}</p> )
            }
            utils::FetchState::Ongoing => {
                html!( <p class={classes!("status")}>{"Fetching..."}</p> )
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                html!(<></>)
            }
            _ => unreachable!(),
        }
    }
}

pub fn projects() -> Html {
    html!( <Projects /> )
}
