use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse, HttpResponseBuilder};
use thiserror::Error;

use tokio::task;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal error")]
    InternalError,
    #[error("Auth error: {0}")]
    AuthError(String),
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(_: mongodb::error::Error) -> Self {
        ApiError::InternalError
    }
}

impl From<bson::de::Error> for ApiError {
    fn from(_: bson::de::Error) -> Self {
        ApiError::InternalError
    }
}

impl From<task::JoinError> for ApiError {
    fn from(_: task::JoinError) -> Self {
        ApiError::InternalError
    }
}
