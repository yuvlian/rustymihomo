use thiserror::Error;

#[derive(Error, Debug)]
pub enum MihomoError {
    #[error("HTTP request failed")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid parameters")]
    InvalidParams,
}
