use std::iter;

mod internals {
    use std::{collections, iter, sync};

    use itertools::Itertools;
    use yew_router::{components::Link, Routable};

    use crate::callout::Callout;
    use crate::{config, Route};

    static EMOJI_ATTR: sync::LazyLock<html5gum::HtmlString> =
        sync::LazyLock::new(|| html5gum::HtmlString(b"emoji".to_vec()));

    static ROUTE_ATTR: sync::LazyLock<html5gum::HtmlString> =
        sync::LazyLock::new(|| html5gum::HtmlString(b"to".to_vec()));

    static TITLE_ATTR: sync::LazyLock<html5gum::HtmlString> =
        sync::LazyLock::new(|| html5gum::HtmlString(b"title".to_vec()));

    static THEME_SET: sync::LazyLock<ThemeSet> = sync::LazyLock::new(|| {
        let ts = syntect::highlighting::ThemeSet::load_defaults();

        ThemeSet {
            dark: ts.themes["base16-mocha.dark"].clone(),
            light: ts.themes["base16-ocean.light"].clone(),
        }
    });

    static SYNTAX_SET: sync::LazyLock<syntect::parsing::SyntaxSet> =
        sync::LazyLock::new(syntect::parsing::SyntaxSet::load_defaults_newlines);

    struct ThemeSet {
        dark: syntect::highlighting::Theme,
        light: syntect::highlighting::Theme,
    }

    fn apply_attributes(
        vtag: &mut yew::virtual_dom::VTag,
        attributes: collections::BTreeMap<
            html5gum::HtmlString,
            html5gum::Spanned<html5gum::HtmlString, ()>,
        >,
    ) -> Option<()> {
        for (key, html5gum::Spanned { value, .. }) in attributes.into_iter() {
            let value = String::from_utf8(value.0).ok()?;

            match key.0.as_slice() {
                b"id" => vtag.add_attribute("id", value),
                b"class" => vtag.add_attribute("class", value),
                b"style" => vtag.add_attribute("style", value),
                b"title" => vtag.add_attribute("title", value),

                b"href" => vtag.add_attribute("href", value),
                b"target" => vtag.add_attribute("target", value),
                b"rel" => vtag.add_attribute("rel", value),

                b"src" => vtag.add_attribute("src", value),
                b"alt" => vtag.add_attribute("alt", value),
                b"width" => vtag.add_attribute("width", value),
                b"height" => vtag.add_attribute("height", value),

                b"type" => vtag.add_attribute("type", value),
                b"value" => vtag.add_attribute("value", value),
                b"placeholder" => vtag.add_attribute("placeholder", value),
                b"name" => vtag.add_attribute("name", value),

                b"checked" if value == "true" => vtag.add_attribute("checked", ""),
                b"disabled" if value == "true" => vtag.add_attribute("disabled", ""),
                b"selected" if value == "true" => vtag.add_attribute("selected", ""),
                b"readonly" if value == "true" => vtag.add_attribute("readonly", ""),
                b"multiple" if value == "true" => vtag.add_attribute("multiple", ""),

                b"role" => vtag.add_attribute("role", value),
                b"aria-label" => vtag.add_attribute("aria-label", value),
                b"aria-hidden" => vtag.add_attribute("aria-hidden", value),
                _ => (),
            }
        }

        Some(())
    }

    pub fn parse(parser: &mut pulldown_cmark::Parser<'_>, prefers_dark: bool) -> Option<yew::Html> {
        match parser.next()? {
            pulldown_cmark::Event::Start(tag) => match tag {
                pulldown_cmark::Tag::Paragraph => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<p>{ for children }</p>))
                }

                pulldown_cmark::Tag::Heading { level, .. } => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));

                    Some(match level {
                        pulldown_cmark::HeadingLevel::H1 => {
                            yew::html!(<h1>{ for children }</h1>)
                        }
                        pulldown_cmark::HeadingLevel::H2 => {
                            yew::html!(<h2>{ for children }</h2>)
                        }
                        pulldown_cmark::HeadingLevel::H3 => {
                            yew::html!(<h3>{ for children }</h3>)
                        }
                        pulldown_cmark::HeadingLevel::H4 => {
                            yew::html!(<h4>{ for children }</h4>)
                        }
                        pulldown_cmark::HeadingLevel::H5 => {
                            yew::html!(<h5>{ for children }</h5>)
                        }
                        pulldown_cmark::HeadingLevel::H6 => {
                            yew::html!(<h6>{ for children }</h6>)
                        }
                    })
                }

                pulldown_cmark::Tag::BlockQuote(_) => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<blockquote>{ for children }</blockquote>))
                }

                pulldown_cmark::Tag::CodeBlock(kind) => {
                    let pulldown_cmark::Event::Text(code) = parser.next()? else {
                        return Some(yew::html!(<pre><code></code></pre>));
                    };
                    parser.next();

                    let pulldown_cmark::CodeBlockKind::Fenced(language) = kind else {
                        return Some(yew::html!(<pre><code>{code.into_string()}</code></pre>));
                    };

                    let Some(syntax) = SYNTAX_SET.find_syntax_by_token(&language) else {
                        return Some(yew::html!(<pre><code>{code.into_string()}</code></pre>));
                    };

                    let mut h = if prefers_dark {
                        syntect::easy::HighlightLines::new(syntax, &THEME_SET.dark)
                    } else {
                        syntect::easy::HighlightLines::new(syntax, &THEME_SET.light)
                    };

                    let html: String = code
                        .lines()
                        .flat_map(|line| h.highlight_line(line, &SYNTAX_SET))
                        .flat_map(|v| {
                            syntect::html::styled_line_to_highlighted_html(
                                &v,
                                syntect::html::IncludeBackground::No,
                            )
                        })
                        .intersperse("\n".to_string())
                        .collect();

                    Some(
                        yew::html!(<pre><code>{yew::Html::from_html_unchecked(html.into())}</code></pre>),
                    )
                }

                pulldown_cmark::Tag::HtmlBlock => {
                    let mut html_block = String::new();

                    while let Some(pulldown_cmark::Event::Html(html)) = parser.next() {
                        html_block.push_str(&html);
                    }

                    let mut tokenizer = html5gum::Tokenizer::new(&html_block);
                    let children = iter::from_fn(move || parse_html(&mut tokenizer));

                    Some(yew::html!(<>{ for children }</>))
                }

                pulldown_cmark::Tag::List(start) => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));

                    if let Some(start) = start {
                        Some(yew::html!(<ol start={start.to_string()}>{ for children }</ol>))
                    } else {
                        Some(yew::html!(<ul>{ for children }</ul>))
                    }
                }

                pulldown_cmark::Tag::Item => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<li>{ for children }</li>))
                }

                pulldown_cmark::Tag::Table(alignments) => todo!(),
                pulldown_cmark::Tag::TableHead => todo!(),
                pulldown_cmark::Tag::TableRow => todo!(),
                pulldown_cmark::Tag::TableCell => todo!(),

                pulldown_cmark::Tag::Emphasis => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<em>{ for children }</em>))
                }

                pulldown_cmark::Tag::Strong => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<strong>{ for children }</strong>))
                }

                pulldown_cmark::Tag::Strikethrough => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<del>{ for children }</del>))
                }

                pulldown_cmark::Tag::Superscript => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<sup>{ for children }</sup>))
                }

                pulldown_cmark::Tag::Subscript => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(yew::html!(<sub>{ for children }</sub>))
                }

                pulldown_cmark::Tag::Link {
                    dest_url, title, ..
                } => {
                    let children = iter::from_fn(|| parse(parser, prefers_dark));
                    Some(
                        yew::html!(<a href={dest_url.into_string()} title={title.into_string()}>{ for children }</a>),
                    )
                }

                pulldown_cmark::Tag::Image {
                    dest_url, title, ..
                } => {
                    let mut alt = String::new();

                    for text in iter::from_fn(|| parse_text(parser)) {
                        alt.push_str(&text);
                    }

                    Some(
                        yew::html!(<img src={dest_url.into_string()} title={title.into_string()} {alt}/>),
                    )
                }

                _ => unreachable!(),
            },

            pulldown_cmark::Event::End(_) => None,

            pulldown_cmark::Event::Text(text) => Some(yew::html!(text.into_string())),

            pulldown_cmark::Event::Code(code) => {
                Some(yew::html!(<code>{code.into_string()}</code>))
            }

            pulldown_cmark::Event::InlineMath(math) => {
                let opts = katex::Opts::builder()
                    .error_color(config::colors::ERROR)
                    .output_type(katex::OutputType::Mathml)
                    .trust(true)
                    .build()
                    .unwrap();

                katex::render_with_opts(&math, opts)
                    .ok()
                    .map(|html| yew::Html::from_html_unchecked(html.into()))
            }

            pulldown_cmark::Event::DisplayMath(math) => {
                let opts = katex::Opts::builder()
                    .display_mode(true)
                    .error_color(config::colors::ERROR)
                    .output_type(katex::OutputType::Mathml)
                    .trust(true)
                    .build()
                    .unwrap();

                katex::render_with_opts(&math, opts)
                    .ok()
                    .map(|html| yew::Html::from_html_unchecked(html.into()))
            }

            pulldown_cmark::Event::InlineHtml(tag) => {
                let mut tokenizer = html5gum::Tokenizer::new(tag.as_ref());
                match tokenizer.next()?.ok()? {
                    html5gum::Token::StartTag(start_tag) => {
                        match start_tag.name.0.as_slice() {
                            b"callout" => {
                                let title = String::from_utf8(
                                    start_tag.attributes[&*TITLE_ATTR].value.0.clone(),
                                )
                                .ok()?;
                                let emoji = String::from_utf8(
                                    start_tag.attributes[&*EMOJI_ATTR].value.0.clone(),
                                )
                                .ok()?;
                                let children = iter::from_fn(|| parse(parser, prefers_dark));

                                return Some(
                                    yew::html!(<Callout {title} {emoji}>{ for children }</Callout>),
                                );
                            }

                            b"internal-link" => {
                                let route = String::from_utf8(
                                    start_tag.attributes[&*ROUTE_ATTR].value.0.clone(),
                                )
                                .ok()?;
                                let route = Route::recognize(&route)?;
                                let children = iter::from_fn(|| parse(parser, prefers_dark));

                                return Some(
                                    yew::html!(<Link<Route> to={route}>{ for children }</Link<Route>>),
                                );
                            }

                            _ => (),
                        }

                        let mut vtag =
                            yew::virtual_dom::VTag::new(String::from_utf8(start_tag.name.0).ok()?);

                        apply_attributes(&mut vtag, start_tag.attributes)?;

                        if !start_tag.self_closing {
                            vtag.add_children(iter::from_fn(|| parse(parser, prefers_dark)));
                        }

                        Some(vtag.into())
                    }

                    html5gum::Token::EndTag(_) => None,

                    html5gum::Token::String(html5gum::Spanned {
                        value: html5gum::HtmlString(bytes),
                        ..
                    }) => Some(yew::html!(String::from_utf8(bytes).ok()?)),

                    _ => parse(parser, prefers_dark),
                }
            }

            pulldown_cmark::Event::SoftBreak => Some(yew::html!("\n")),

            pulldown_cmark::Event::HardBreak => Some(yew::html!(<br/>)),

            pulldown_cmark::Event::Rule => Some(yew::html!(<hr/>)),

            pulldown_cmark::Event::TaskListMarker(checked) => {
                Some(yew::html!(<input type="checkbox" disabled=true {checked}/>))
            }

            _ => unreachable!(),
        }
    }

    fn parse_html<R: html5gum::Reader>(
        tokenizer: &mut html5gum::Tokenizer<R>,
    ) -> Option<yew::Html> {
        match tokenizer.next()?.ok()? {
            html5gum::Token::StartTag(start_tag) => {
                match start_tag.name.0.as_slice() {
                    b"callout" => {
                        let title =
                            String::from_utf8(start_tag.attributes[&*TITLE_ATTR].value.0.clone())
                                .ok()?;
                        let emoji =
                            String::from_utf8(start_tag.attributes[&*EMOJI_ATTR].value.0.clone())
                                .ok()?;
                        let children = iter::from_fn(|| parse_html(tokenizer));

                        return Some(
                            yew::html!(<Callout {title} {emoji}>{ for children }</Callout>),
                        );
                    }

                    b"internal-link" => {
                        let route =
                            String::from_utf8(start_tag.attributes[&*ROUTE_ATTR].value.0.clone())
                                .ok()?;
                        let route = Route::recognize(&route)?;
                        let children = iter::from_fn(|| parse_html(tokenizer));

                        return Some(
                            yew::html!(<Link<Route> to={route}>{ for children }</Link<Route>>),
                        );
                    }

                    _ => (),
                }

                let mut vtag =
                    yew::virtual_dom::VTag::new(String::from_utf8(start_tag.name.0).ok()?);

                apply_attributes(&mut vtag, start_tag.attributes)?;

                if !start_tag.self_closing {
                    vtag.add_children(iter::from_fn(|| parse_html(tokenizer)));
                }

                Some(vtag.into())
            }

            html5gum::Token::EndTag(_) => None,

            html5gum::Token::String(html5gum::Spanned {
                value: html5gum::HtmlString(bytes),
                ..
            }) => Some(yew::html!(String::from_utf8(bytes).ok()?)),

            _ => parse_html(tokenizer),
        }
    }

    fn parse_text<'input>(
        parser: &mut pulldown_cmark::Parser<'input>,
    ) -> Option<pulldown_cmark::CowStr<'input>> {
        match parser.next()? {
            pulldown_cmark::Event::End(_) => None,
            pulldown_cmark::Event::Text(text) => Some(text),
            pulldown_cmark::Event::Code(code) => Some(code),
            pulldown_cmark::Event::InlineMath(math) => Some(math),
            pulldown_cmark::Event::DisplayMath(math) => Some(math),
            pulldown_cmark::Event::InlineHtml(tag) => Some(tag),
            _ => Some(pulldown_cmark::CowStr::Borrowed("")),
        }
    }
}

pub fn parse<'input>(
    text: &'input str,
    prefers_dark: bool
) -> iter::FromFn<impl FnMut() -> Option<yew::Html> + 'input> {
    let options = pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES
        | pulldown_cmark::Options::ENABLE_MATH
        | pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION
        | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
        | pulldown_cmark::Options::ENABLE_SUBSCRIPT
        | pulldown_cmark::Options::ENABLE_SUPERSCRIPT
        | pulldown_cmark::Options::ENABLE_TABLES
        | pulldown_cmark::Options::ENABLE_TASKLISTS;
    let mut parser = pulldown_cmark::Parser::new_ext(text, options);

    iter::from_fn(move || internals::parse(&mut parser, prefers_dark))
}
