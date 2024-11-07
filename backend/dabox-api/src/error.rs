use crate::prelude::*;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use dabox_core::error::DaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal server error")]
    InternalServerError,
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found")]
    NotFound,
}

impl From<DaError> for ApiError {
    fn from(err: DaError) -> Self {
        match err {
            DaError::DirectoryNotFound(_) => Self::NotFound,
            DaError::AccessDenied { requested_by, .. } => Self::Forbidden(format!(
                "the user {requested_by} is not allowed to access the resource"
            )),
            _ => {
                error!("Unexpected error: {err}");
                Self::InternalServerError
            }
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden(_) => (StatusCode::FORBIDDEN, "Forbidden").into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
