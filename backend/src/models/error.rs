use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};
use mongodb::error::Error;

use tokio::task;

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "internal error")]
    InternalError,
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }
}

impl From<mongodb::error::Error> for ApiError {
    fn from(_: Error) -> Self {
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
