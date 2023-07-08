use std::{
    fmt::{self, Display, Formatter},
    io,
};

pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// An Error returned by the API
    ApiError(u16, String),
    /// An Error not related to the API
    RequestError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiError(status, msg) => {
                write!(f, "API responded with status {} error: {}", status, msg)
            }
            Error::RequestError(msg) => write!(f, "Request error: {}", msg),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        match value {
            ureq::Error::Status(status, response) => {
                Error::ApiError(status, response.into_string().unwrap_or_default())
            }
            ureq::Error::Transport(e) => Error::RequestError(e.to_string()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::RequestError(value.to_string())
    }
}
