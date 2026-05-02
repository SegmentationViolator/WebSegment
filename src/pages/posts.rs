use serde::Deserialize;

use crate::card::Card;
use crate::utils;
use crate::Route;

#[derive(Deserialize)]
struct Post {
    title: String,
    datetime: String,
    filename: String,
}

struct PostList {
    posts: Vec<Post>,
    fetch_state: utils::FetchState<reqwest::Error>,
}

impl yew::Component for PostList {
    type Message = utils::Message<Vec<Post>, utils::Never, reqwest::Error>;
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

                    match reqwest::get(format!("{base}/posts.json"))
                        .await
                        .and_then(|response| response.error_for_status())
                    {
                        Err(error) => {
                            if let Some(reqwest::StatusCode::NOT_FOUND) = error.status() {
                                return utils::Message::SetContent(Vec::new());
                            }

                            utils::Message::SetState(utils::FetchState::Error(error))
                        }
                        Ok(response) => match response.json().await {
                            Err(error) => utils::Message::SetState(utils::FetchState::Error(error)),
                            Ok(posts) => utils::Message::SetContent(posts),
                        },
                    }
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
            _ => unreachable!(), // Message::UpdateData is never sent
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        match &self.fetch_state {
            utils::FetchState::Complete => {
                if self.posts.is_empty() {
                    return yew::html! {
                        <p>{"Nothing to see here."}</p>
                    };
                }

                let cards = self.posts.iter().map(|post| {
                    yew::html!(
                        <Card
                            title={post.title.clone()}
                            url={utils::Url::Internal(Route::Post { filename: post.filename.clone() })}
                            subtext={utils::format_datetime(post.datetime.clone())}
                        />
                    )
                });

                yew::html! {
                    <div class={yew::classes!("card-grid")}>
                        { for cards }
                    </div>
                }
            }
            utils::FetchState::Error(error) => {
                yew::html! {
                    <p class={yew::classes!("status", "error")}>{error.to_string()}</p>
                }
            }
            utils::FetchState::Ongoing => {
                yew::html! {
                    <p class={yew::classes!("status")}>{"Fetching..."}</p>
                }
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                yew::html!(<></>)
            }
            _ => unreachable!(), // FetchState::NotFound is never set as fetch_state
        }
    }
}

pub fn posts() -> yew::Html {
    yew_hooks::use_title("Projects".to_string());
    yew::html!(<PostList />)
}
