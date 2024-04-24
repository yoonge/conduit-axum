use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
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
    AppError, PAGE_SIZE,
};
use crate::db::{NewUser, User};

pub async fn login(
    State(_pool): State<Pool<Postgres>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>, AppError> {
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

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User login succeed."));
    res.insert("token".to_string(), json!(token));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, AppError> {
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

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User register succeed."));
    res.insert("token".to_string(), json!(token));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn query_user(pool: Pool<Postgres>, user_id: Uuid) -> Result<User, AppError> {
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
) -> Result<Json<Value>, AppError> {
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

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User query succeed."));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_users(
    _claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", _claims);
    println!("\nQuery Args: {:?}\n", args);
    let page = args
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()?;
    let offset = (page - 1) * PAGE_SIZE;

    let users: Vec<User> = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, update_at, username
            from users
            order by create_at desc
            limit $1 offset $2
        "#
    )
    .bind(&page)
    .bind(&offset)
    .fetch_all(&pool)
    .await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Users query succeed."));
    res.insert("page".to_string(), json!(&page));
    res.insert("users".to_string(), json!(&users));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_user_settings(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let user = query_user(pool, claims.cuid).await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User settings query succeed."));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}
