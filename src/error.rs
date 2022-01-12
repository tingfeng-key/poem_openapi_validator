use poem::{ http::StatusCode};
use poem::error::ResponseError;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ValidatorError {
    pub message: String,
}

impl ResponseError for ValidatorError {
    fn status(&self) -> StatusCode {
        StatusCode::OK
    }
}