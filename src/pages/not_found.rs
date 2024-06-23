use yew::prelude::*;

pub fn not_found() -> Html {
    html! {
        <div class="not-found">
            <h1>{"404"}</h1>
            <h2>{"Page Not Found"}</h2>
        </div>
    }
}
