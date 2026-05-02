use yew::classes;
use yew_router::components::Link;

use crate::config;
use crate::Route;

#[stylist::yew::styled_component(NavigationBar)]
pub fn navigation_bar() -> yew::Html {
    let current_route: Route = yew_router::hooks::use_route().unwrap();
    let navigator = yew_router::hooks::use_navigator().unwrap();

    let disabled_link = css!("pointer-events: none;");

    let pages = Route::DISPLAYABLE
        .iter()
        .map(|route| {
            let mut class_list = classes!("nav-link");
            if current_route == *route {
                class_list.push("active");
                class_list.push(disabled_link.clone());
            }

            yew::html! {
                <li>
                    <Link<Route> classes={class_list} to={route.clone()}>
                        <small> {route.to_string()} </small>
                    </Link<Route>>
                </li>
            }
        })
        .collect::<yew::Html>();

    let mut class_list = classes!("nav-head");
    if cfg!(feature = "pri-demon-th") {
        class_list.push(css!(r#"
            background-clip: text;
            background-image: linear-gradient(to top, #770088 25%, #004CFF 35%, #028121 45%, #FFEE00 55%, #FF8D00 65%, #E50000 75%);
            color: transparent;
        "#));
    }

    yew::html! {
        <div class={yew::classes!("nav-bar")}>
            <h2 onclick={move |_| { if current_route != Route::Home { navigator.push(&Route::Home); } } }  class={class_list}>
                { config::TITLE }
            </h2>
            <ui class={yew::classes!("nav-links")}>
                { pages }
            </ui>
        </div>
    }
}
