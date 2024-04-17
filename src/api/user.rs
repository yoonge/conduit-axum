use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use super::ApiError;
use conduit_axum::db::User;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, ApiError> {
    let user = sqlx::query_as!(
        User,
        "insert into users (email, password, username) values ($1, $2, $3) returning *",
        &new_user.email,
        &new_user.password,
        &new_user.username
    )
    // .bind(&new_user.email)
    // .bind(&new_user.password)
    // .bind(&new_user.username)
    .fetch_one(&pool)
    .await?;

    println!("{:?}", user);

    Ok(Json(user))
}

pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let user = sqlx::query_as!(User, "select * from users where id = $1", user_id)
        .fetch_one(&pool)
        .await?;

    println!("{:?}", user);

    Ok(Json(user))
}

pub async fn get_users(State(pool): State<Pool<Postgres>>) -> Result<Json<Vec<User>>, ApiError> {
    let users = sqlx::query_as!(User, "select * from users")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", users);

    Ok(Json(users))
}
