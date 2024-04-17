use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod user;

pub enum ApiError {
    INTERNAL(anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong."),
        )
            .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::INTERNAL(err.into())
    }
}
