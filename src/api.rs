use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod user;
pub mod utils;

pub enum AppError {
    INTERNAL(anyhow::Error),
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
        Self::INTERNAL(err.into())
    }
}
