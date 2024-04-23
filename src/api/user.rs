use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use jsonwebtoken::{encode, Header};
use serde_json::{json, Map, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{
    utils::{
        jwt::{AuthError, AuthPayload, Claims, KEYS},
        password,
    },
    AppError, AppResponse,
};
use crate::db::{NewUser, User};

pub async fn login(
    State(_pool): State<Pool<Postgres>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AppResponse<Value>>, AppError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::Auth(AuthError::MissingCredentials));
    }

    let hashed_password = password::hash(payload.password).await?;
    let user: User = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, update_at, username
            from users
            where email = $1 and password = $2
        "#
    )
    .bind(&payload.email)
    .bind(&hashed_password)
    .fetch_one(&_pool)
    .await
    .map_err(|_| AppError::Auth(AuthError::InvalidCredentials))?;

    let user_clone = user.clone();
    let claims = Claims::new(user_clone._id, user_clone.nickname, user_clone.username);
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    let mut data = Map::new();
    data.insert("token".to_string(), json!(token));
    data.insert("user".to_string(), json!(&user));

    let res = AppResponse {
        code: StatusCode::OK.into(),
        data: json!(data),
        msg: "User login succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<AppResponse<Value>>, AppError> {
    let hashed_password = password::hash(new_user.password).await?;
    let user: User = sqlx::query_as(
        r#"
            insert into users (email, password, username)
            values ($1, $2, $3)
            returning _id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, password, phone, position, update_at, username
        "#,
    )
    .bind(&new_user.email)
    .bind(&hashed_password)
    .bind(&new_user.username)
    .fetch_one(&pool)
    .await?;

    let user_clone = user.clone();
    let claims = Claims::new(user_clone._id, user_clone.nickname, user_clone.username);
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    let mut data = Map::new();
    data.insert("token".to_string(), json!(token));
    data.insert("user".to_string(), json!(&user));

    let res = AppResponse {
        code: StatusCode::CREATED.into(),
        data: json!(data),
        msg: "User create succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn _query_user(
    _claims: Claims,
    pool: Pool<Postgres>,
    user_id: Uuid,
) -> Result<User, AppError> {
    println!("\n{:?}\n", _claims);

    let user: User = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, update_at, username
            from users
            where _id = $1
        "#
    ).bind(&user_id)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}

pub async fn get_user(
    _claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
) -> Result<Json<AppResponse<Value>>, AppError> {
    println!("\n{:?}\n", _claims);

    let user: User = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, update_at, username
            from users
            where username = $1
        "#
    ).bind(username)
    .fetch_one(&pool)
    .await?;

    let mut data = Map::new();
    data.insert("user".to_string(), json!(&user));

    let res = AppResponse {
        code: StatusCode::OK.into(),
        data: json!(data),
        msg: "User query succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn get_users(
    _claims: Claims,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<AppResponse<Value>>, AppError> {
    println!("\n{:?}\n", _claims);

    let users: Vec<User> = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, update_at, username
            from users
        "#
    )
    .fetch_all(&pool)
    .await?;

    let mut data = Map::new();
    data.insert("users".to_string(), json!(&users));

    let res = AppResponse {
        code: StatusCode::OK.into(),
        data: json!(data),
        msg: "Users query succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}
