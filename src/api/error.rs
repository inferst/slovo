#[derive(thiserror::Error, Debug)]
pub enum Error {
    Storate(#[from] gloo::storage::errors::StorageError),
    Request(#[from] reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
