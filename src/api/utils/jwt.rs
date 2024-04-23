use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub cuid: Uuid,
    pub exp: usize,
    pub nickname: String,
    pub username: String,
}

impl Claims {
    pub fn new(cuid: Uuid, nickname: String, username: String) -> Self {
        let exp = SystemTime::now() + Duration::from_secs(24 * 60 * 60);
        let exp = exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        Self {
            cuid,
            exp,
            nickname,
            username,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

// #[derive(Debug, Serialize)]
// pub struct AuthBody {
//     token: String,
//     token_type: String,
// }

// impl AuthBody {
//     pub fn new(token: String) -> Self {
//         Self {
//             token,
//             token_type: "Bearer".to_string(),
//         }
//     }
// }

#[derive(Debug, Serialize)]
pub enum AuthError {
    InvalidCredentials,
    InvalidToken,
    MissingCredentials,
    TokenCreation,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials."),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token."),
            Self::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials."),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error."),
        };
        let body = Json(json!({ "err": msg }));
        (code, body).into_response()
    }
}
