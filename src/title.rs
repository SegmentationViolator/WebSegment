#[derive(Clone, PartialEq, yew::Properties)]
pub struct Props {
    pub text: String,
}

#[yew::function_component(Title)]
pub fn title(props: &Props) -> yew::Html {
    yew_hooks::use_title(props.text.clone());
    yew::html!(<></>)
}
