use crate::Route;

pub enum FetchState {
    Complete,
    Error(String),
    NotFound,
    Ongoing,
    Pending,
}

pub enum Message<T> {
    FetchData,
    SetContent(T),
    SetState(FetchState),
}

#[derive(Clone, PartialEq)]
pub enum Url {
    External(String),
    Internal(Route),
}
