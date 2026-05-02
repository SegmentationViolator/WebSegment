#[stylist::yew::styled_component(Home)]
pub fn home() -> yew::Html {
    let break_condition = stylist::yew::use_media_query("(max-width: 508px)");
    let hi_image_url = if stylist::yew::use_media_query("(prefers-color-scheme: dark)") {
        "/assets/hi_dark.gif"
    } else {
        "/assets/hi_light.gif"
    };

    yew::html! {
        <>
            <div
                class={css!("width: 100%; height: 25vh; background-position: center; background-repeat: no-repeat;")}
                style={format!("background-image: url({})", hi_image_url)}
            >
            </div>

            <p class={css!("font-size: 1.5rem; hyphens: none; line-break: normal; text-align: center;")}>
                {"I am SegV, "}

                if break_condition {
                    <br/>
                }

                {"a hobbyist programmer."}
            </p>

            <br/>

            <p style="text-align: center;">
                {"I am particularly interested in systems programming, computers graphics and game development. "}
                {"I consider my expertise level to be intermediate and have a lot to learn."}
            </p>

            <br/><br/>
        </>
    }
}

pub fn home() -> yew::Html {
    yew_hooks::use_title("Home".to_string());
    yew::html!(<Home/>)
}
