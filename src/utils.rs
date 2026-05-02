use std::sync;

use wasm_bindgen::prelude::Upcast;
use web_sys::js_sys;

use crate::Route;

// empty array to force the use of runtime's default locale
static DATETIME_FORMAT_LOCALES: sync::LazyLock<js_sys::Array> = sync::LazyLock::new(js_sys::Array::new);

static DATETIME_FORMAT_OPTIONS: sync::LazyLock<js_sys::Object> = sync::LazyLock::new(|| {
    let options = js_sys::Intl::DateTimeFormatOptions::new();
    options.set_date_style(js_sys::Intl::DateTimeStyle::Short);
    options.set_time_style(js_sys::Intl::DateTimeStyle::Short);

    options.into()
});

pub enum FetchState<E> {
    Complete,
    Error(E),
    NotFound,
    Ongoing,
    Pending,
}

pub enum Message<C, D, E> {
    FetchData,
    SetContent(C),
    SetState(FetchState<E>),
    UpdateData(D),
}

pub enum Never {}

#[derive(Clone, PartialEq)]
pub enum Url {
    External(String),
    Internal(Route),
}

pub fn format_datetime(datetime: String) -> String {
    let date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&datetime));

    if date.get_time().is_nan() {
        return datetime;
    }

    let date: &wasm_bindgen::JsValue = date.upcast();
    let formatter = js_sys::Intl::DateTimeFormat::new(&DATETIME_FORMAT_LOCALES, &DATETIME_FORMAT_OPTIONS);

    formatter.format().call(&wasm_bindgen::JsValue::NULL, (date,)).ok().and_then(|js_string| js_string.as_string()).unwrap()
}
