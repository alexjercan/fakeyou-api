use std::{
    fmt::{self, Display, Formatter},
    io,
};

pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::RequestError(msg) => write!(f, "Request error: {}", msg),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequestError(value.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::RequestError(value.to_string())
    }
}
