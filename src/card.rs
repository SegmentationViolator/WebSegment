use crate::utils;

const BASE_DELAY: u32 = 900;
const MAX_RETRIES: i32 = 3;

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Props {
    pub title: String,
    pub url: utils::Url,
    #[prop_or_default]
    pub subtext: Option<String>,
    #[prop_or_default]
    pub image_url: Option<String>,
}

#[yew::function_component(Card)]
pub fn card(properties: &Props) -> yew::Html {
    let location = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .location()
        .unwrap();
    let navigator = yew_router::hooks::use_navigator().unwrap();

    let retry_count = yew_hooks::use_counter(0);

    let on_error = {
        let retry_count = retry_count.clone();

        yew::Callback::from(move |_| {
            if *retry_count < MAX_RETRIES {
                let exponent = *retry_count as u32;
                let delay = BASE_DELAY * 2_u32.pow(exponent);

                let retry_count = retry_count.clone();

                gloo_timers::callback::Timeout::new(delay, move || {
                    retry_count.increase();
                })
                .forget();
            }
        })
    };

    let inner = yew::html! {
        <>
            <div class={yew::classes!("card-head")}>
                <h3>{&properties.title}</h3>
                if let Some(subtext) = &properties.subtext {
                    <small class={yew::classes!("card-subtext")}>{subtext}</small>
                }
            </div>

            if let Some(image_url) = &properties.image_url {
                <img class={yew::classes!("card-image")} src={format!("{}?retries={}", image_url, *retry_count)} onerror={on_error}/>
            }
        </>
    };

    match &properties.url {
        utils::Url::External(url) => {
            let url = url.clone();
            yew::html! {
                <div onclick={move |_| { let _ = location.set_href(&url); } } class={yew::classes!("card", "hover-scale")}>
                    {inner}
                </div>
            }
        }
        utils::Url::Internal(route) => {
            let route = route.clone();
            yew::html! {
                <div onclick={move |_| { navigator.push(&route); } } class={yew::classes!("card", "hover-scale")}>
                    {inner}
                </div>
            }
        }
    }
}
