use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub mod topic;
pub mod user;
pub mod utils;

use self::utils::jwt::AuthError;

#[derive(Debug, Serialize)]
pub struct AppResponse<T: Serialize> {
    code: u16,
    data: T,
    msg: String,
}

pub enum AppError {
    Auth(AuthError),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong."),
        )
            .into_response()
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
