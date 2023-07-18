use thiserror::Error;

pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send request: `{0}`")]
    RequestFailed(String),
    #[error("Failed to parse response: `{0}`")]
    ParseError(String),
    #[error("Bad request, something was wrong with your request.")]
    BadRequest,
    #[error("Unauthorized, please check your API key.")]
    Unauthorized,
    #[error("Too many requests, please wait a bit before trying again.")]
    TooManyRequests,
    #[error("Internal server error, please try again later.")]
    InternalServerError,
    #[error("Unknown error: `{0}`")]
    Unknown(u16),
    #[error("TTS job failed: `{0}`")]
    JobFailed(String),
}
