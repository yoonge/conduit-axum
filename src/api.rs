use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}, Json,
};
use serde_json::json;

pub mod common;
pub mod tag;
pub mod topic;
pub mod user;
pub mod utils;

use self::utils::jwt::AuthError;

pub static PAGE_SIZE: i32 = 10;

pub enum AppError {
    Auth(AuthError),
    Duplicate(anyhow::Error),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Auth(err) => err.into_response(),
            Self::Duplicate(err) => (
                StatusCode::CONFLICT,
                Json(json!({
                    "code": StatusCode::CONFLICT.as_u16(),
                    "msg": format!("Duplicate entry: {}.", err),
                })),
            )
                .into_response(),
            Self::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    "msg": format!("Something went wrong: {}.", err),
                })),
            )
                .into_response(),
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}
