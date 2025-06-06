#[derive(thiserror::Error, Debug)]
pub enum Error {
    EmptyToken,
    Request(#[from] reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
