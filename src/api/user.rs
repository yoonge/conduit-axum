use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, to_value, Map, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{utils::password, AppError, AppResponse};
use crate::db::{NewUser, User};

pub async fn create_user(
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

    let mut data = Map::new();
    data.insert("user".to_string(), json!(&user));
    let data = to_value(data)?;

    let res = AppResponse {
        code: StatusCode::CREATED.into(),
        data,
        msg: "User create succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn _query_user(pool: Pool<Postgres>, user_id: Uuid) -> Result<User, AppError> {
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
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
) -> Result<Json<AppResponse<Value>>, AppError> {
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
    let data = to_value(data)?;

    let res = AppResponse {
        code: StatusCode::OK.into(),
        data,
        msg: "User query succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn get_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<AppResponse<Value>>, AppError> {
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
    let data = to_value(data)?;

    let res = AppResponse {
        code: StatusCode::OK.into(),
        data,
        msg: "Users query succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn verify_pwd(password: String) -> Result<String, AppError> {
    let pwd = password::hash(password.clone()).await?;
    let res = password::verify(password, pwd).await?;

    Ok(res.to_string())
}
