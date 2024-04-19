use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::api::{utils::password, AppError};
use crate::db::User;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, AppError> {
    let hashed_password = password::hash(new_user.password).await?;
    let user: User = sqlx::query_as(
        r#"
            insert into users (email, password, username)
            values ($1, $2, $3)
            returning *
        "#,
    )
    .bind(&new_user.email)
    .bind(&hashed_password)
    .bind(&new_user.username)
    .fetch_one(&pool)
    .await?;

    println!("{:?}", user);

    Ok(Json(user))
}

pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
) -> Result<Json<User>, AppError> {
    let user: User = sqlx::query_as(
        r#"
            select id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, username
            from users
            where username = $1
        "#
    ).bind(username)
    .fetch_one(&pool)
    .await?;

    println!("{:?}", user);

    Ok(Json(user))
}

pub async fn get_users(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<User>>, AppError> {
    let users: Vec<User> = sqlx::query_as(
        r#"
            select id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, username
            from users
        "#
    )
    .fetch_all(&pool)
    .await?;

    println!("{:?}", users);

    Ok(Json(users))
}

pub async fn verify_pwd(password: String) -> Result<String, AppError> {
    let pwd = password::hash(password.clone()).await?;
    let res = password::verify(password, pwd).await?;

    Ok(res.to_string())
}
