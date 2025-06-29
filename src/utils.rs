use crate::Route;

pub enum FetchState {
    Complete,
    Error(String),
    NotFound,
    Ongoing,
    Pending,
}

pub enum Message<T, D> {
    FetchData,
    SetContent(T),
    SetState(FetchState),
    UpdateData(D),
}

pub enum Never {}

#[derive(Clone, PartialEq)]
pub enum Url {
    External(String),
    Internal(Route),
}
