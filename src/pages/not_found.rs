use crate::title::Title;

pub fn not_found() -> yew::Html {
    yew::html! {
        <>
            <Title text="Not Found" />

            <div
                class={stylist::css!(
                    "align-items: center; display: flex; flex-direction: column; height: 100%; justify-content: center;"
                )}
            >
                <h1 class={stylist::css!("font-size: 10em;")}>{"404"}</h1>
                <h2>{"Page Not Found"}</h2>
            </div>
        </>
    }
}
