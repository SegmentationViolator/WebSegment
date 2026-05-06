use yew_router::components::Redirect;

use crate::title::Title;
use crate::utils;
use crate::Route;

mod markdown;

const POST_DATA_FIELDS: usize = 3;

struct Post {
    title: String,
    datetime: String,
    body: String,
}

struct InternalPostView {
    fetch_state: utils::FetchState<reqwest::Error>,
    filename: String,
    post: Option<Post>,
}

#[derive(PartialEq, yew::Properties)]
struct InternalProps {
    pub filename: String,
    pub prefers_dark: bool,
}

#[derive(PartialEq, yew::Properties)]
struct Props {
    pub filename: String,
}

impl yew::Component for InternalPostView {
    type Message = utils::Message<Post, String, reqwest::Error>;
    type Properties = InternalProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            fetch_state: utils::FetchState::Pending,
            filename: ctx.props().filename.clone(),
            post: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            utils::Message::FetchData => {
                let filename = ctx.props().filename.clone();

                ctx.link().send_future(async move {
                    let base = web_sys::window().unwrap().location().origin().unwrap();

                    let post_data = match reqwest::get(format!("{base}/files/{filename}"))
                        .await
                        .and_then(|response| response.error_for_status())
                    {
                        Err(error) => {
                            if let Some(reqwest::StatusCode::NOT_FOUND) = error.status() {
                                return utils::Message::SetState(utils::FetchState::NotFound);
                            }

                            return utils::Message::SetState(utils::FetchState::Error(error));
                        }
                        Ok(response) => match response.text().await {
                            Err(error) => {
                                return utils::Message::SetState(utils::FetchState::Error(error))
                            }
                            Ok(text) => text,
                        },
                    };

                    let mut post_data = post_data.splitn(POST_DATA_FIELDS, '\n');

                    let title = post_data.next().unwrap().to_string();
                    let datetime = post_data.next().unwrap().to_string();
                    let body = post_data.next().unwrap().to_string();

                    utils::Message::SetContent(Post {
                        title,
                        datetime,
                        body,
                    })
                });

                self.fetch_state = utils::FetchState::Ongoing;
                true
            }
            utils::Message::SetContent(post) => {
                let _ = self.post.insert(post);

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
        let prefers_dark = stylist::yew::use_media_query("(prefers-color-scheme: dark)");

        match &self.fetch_state {
            utils::FetchState::Complete => {
                let filename = ctx.props().filename.clone();

                if self.filename != filename {
                    let link = ctx.link();
                    link.send_message(utils::Message::UpdateData(filename));
                    link.send_message(utils::Message::SetState(utils::FetchState::Pending));
                    return yew::html!(<></>);
                }

                let post = self
                    .post
                    .as_ref()
                    .expect("body shouldn't be None while fetch_state is Complete");
                let contents = markdown::parse(&post.body, ctx.props().prefers_dark);

                yew::html! {
                    <>
                        <Title title={format!("Post | {}", post.title.clone())} />
                        <div class={yew::classes!("post")}>
                            { for contents }
                        </div>
                    </>
                }
            }
            utils::FetchState::NotFound => yew::html!( <Redirect<Route> to={Route::NotFound} /> ),
            utils::FetchState::Error(error) => {
                yew::html!(<p class={yew::classes!("status", "error")}>{error.to_string()}</p>)
            }
            utils::FetchState::Ongoing => {
                yew::html!(<p class={yew::classes!("status")}>{"Fetching..."}</p>)
            }
            utils::FetchState::Pending => {
                ctx.link().send_message(utils::Message::FetchData);
                yew::html!(<></>)
            }
        }
    }
}

#[stylist::yew::styled_component(PostView)]
fn post_view(properties: &Props) -> yew::Html {
    let prefers_dark = stylist::yew::use_media_query("(prefers-color-scheme: dark)");

    yew::html!(<InternalPostView filename={properties.filename.clone()} {prefers_dark} />)
}

pub fn post(filename: String) -> yew::Html {
    yew::html!(<PostView filename={filename}/>)
}
