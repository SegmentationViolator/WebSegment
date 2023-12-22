#[derive(Debug)]
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
