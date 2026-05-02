use crate::config;

#[yew::function_component(Footer)]
pub fn footer() -> yew::Html {
    yew::html! {
        <div class={yew::classes!("footer")}>
            <p>
                {"Made with "}
                <span style="white-space: nowrap;">
                    <a href="https://yew.rs/">{"Yew"}</a>
                    <span class={yew::classes!("separator")}>{"|"}</span>
                    <a href={format!("https://github.com/{}", config::GITHUB_USERNAME)}>
                        <i class={yew::classes!("fa-brands", "fa-github")}></i>
                    </a>
                    <span class={yew::classes!("separator")}>{"|"}</span>
                    <a href={format!("mailto:{}", config::EMAIL)}>
                        <i class={yew::classes!("fa-solid", "fa-envelope")}></i>
                    </a>
                </span>
            </p>
            <a href={format!("https://github.com/{}/{}", config::GITHUB_USERNAME, config::REPOSITORY_NAME)}>{"Source Code"}</a>
        </div>
    }
}
