use std::borrow;

use crate::config;

#[derive(Clone, PartialEq, yew::Properties)]
pub struct Props {
    pub title: String,
    pub emoji: String,
    pub children: yew::Children,
}

#[stylist::yew::styled_component(Callout)]
pub fn callout(properties: &Props) -> yew::Html {
    let emoji_url = format!("{}/{}.webp", config::EMOJI_VENDOR_URL, properties.emoji,);

    let prefers_dark = stylist::yew::use_media_query("(prefers-color-scheme: dark)");

    let background = if prefers_dark {
        borrow::Cow::Owned(format!("hwb(from {} h 15% b)", config::colors::FOREGROUND))
    } else {
        borrow::Cow::Borrowed(config::colors::PRIMARY)
    };

    yew::html! {
    <div
        class={css!(
            r#"
                align-items: center;
                display: flex;
                gap: 1em;
                padding: 1.5em;
                border: 1px ${border_color} solid;
                border-radius: 1.25em;
                background: ${background};
            "#,
            border_color = config::colors::PRIMARY,
            background = background,
        )}
    >
        <img
            class={css!(r#"
                width: clamp(48px, 8vw, 64px);
                height: clamp(48px, 8vw, 64px);
                flex-shrink: 0;
            "#)}
            src={emoji_url}
        />
        <div class={css!("flex: 1;")}>
            <h3 class={css!("margin: 0 0 2px 0;")}>{ &properties.title }</h3>
            <p class={css!("margin: 0;")}>{ for properties.children.iter() }</p>
        </div>
    </div>
    }
}
