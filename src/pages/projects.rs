use web_sys::js_sys::Date;

use serde::Deserialize;

use crate::card::Card;
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
    fetch_state: utils::FetchState<String>,
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
                title={self.name.clone()}
                url={utils::Url::External(url)}
                {image_url}
            />
        )
    }
}

impl yew::Component for ProjectList {
    type Message = utils::Message<Vec<Project>, utils::Never, String>;
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
                            utils::Message::SetState(utils::FetchState::Error(error.to_string()))
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
                                Err(error) => utils::Message::SetState(utils::FetchState::Error(
                                    error.to_string(),
                                )),
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
                        <p>{"Nothing to see here."}</p>
                    };
                }

                let cards = self.projects.iter().map(|project| project.to_card());

                yew::html! {
                    <div class={yew::classes!("card-grid")}>
                        { for cards }
                    </div>
                }
            }
            utils::FetchState::Error(error_message) => {
                yew::html!(<p class={yew::classes!("status", "error")}>{error_message}</p>)
            }
            utils::FetchState::Ongoing => {
                yew::html!(<p class={yew::classes!("status")}>{"Fetching..."}</p>)
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                yew::html!(<></>)
            }
            _ => unreachable!(), // FetchState::NotFound is never set as fetch_state
        }
    }
}

pub fn projects() -> yew::Html {
    yew_hooks::use_title("Projects".to_string());
    yew::html!(<ProjectList />)
}
