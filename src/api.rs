use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub mod topic;
pub mod user;
pub mod utils;

#[derive(Debug, Serialize)]
pub struct AppResponse<T: Serialize> {
    code: u16,
    data: T,
    msg: String,
}

impl<T: Serialize> AppResponse<T> {
    pub fn new(code: u16, data: T, msg: String) -> Self {
        Self { code, data, msg }
    }
}

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
