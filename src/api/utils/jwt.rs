use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

impl Claims {
    fn new(sub: String, exp: i64) -> Self {
        Self { sub, exp }
    }
}

#[derive(Debug, Serialize)]
pub enum AuthError {
    InvalidToken,
    MissingCredentials,
    TokenCreation,
    WrongCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token."),
            Self::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials."),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error."),
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials."),
        };
        let body = Json(json!({ "err": msg }));
        (code, body).into_response()
    }
}
