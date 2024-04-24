use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod topic;
pub mod user;
pub mod utils;

use self::utils::jwt::AuthError;

pub static PAGE_SIZE: i32 = 10;

pub enum AppError {
    Auth(AuthError),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Auth(err) => err.into_response(),
            Self::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}.", err),
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
