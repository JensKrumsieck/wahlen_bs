use std::collections::HashMap;

use axum::{
    Json,
    http::{StatusCode, header::WWW_AUTHENTICATE},
    response::IntoResponse,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("request path not found")]
    NotFound,
    #[error("error in request body")]
    UnprocessableEntity {
        errors: HashMap<String, Vec<String>>,
    },
    #[error("database error")]
    Sql(#[from] sqlx::Error),
    #[error("something went wrong")]
    Any(#[from] anyhow::Error),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Error::Sql(_) | Error::Any(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<String, Vec<String>>,
                }
                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    [(WWW_AUTHENTICATE, "Token")],
                    self.to_string(),
                )
                    .into_response();
            }
            _ => (),
        }
        (self.status_code(), Json(self.to_string())).into_response()
    }
}