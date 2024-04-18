use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;
use uuid::Uuid;

use super::ApiError;
use conduit_axum::db::{date_formatter, User};
use conduit_axum::password;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryUser {
    pub id: Uuid,
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    #[serde(with = "date_formatter")]
    pub create_at: OffsetDateTime,
    pub email: String,
    pub favorite: Vec<Uuid>,
    pub gender: i16,
    pub nickname: String,
    pub phone: String,
    pub position: String,
    pub username: String,
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, ApiError> {
    let hashed_password = password::hash(new_user.password).await?;
    let user = sqlx::query_as!(
        User,
        r#"
            insert into users (email, password, username)
            values ($1, $2, $3)
            returning *
        "#,
        &new_user.email,
        &hashed_password,
        &new_user.username
    )
    .fetch_one(&pool)
    .await?;

    println!("{:?}", user);

    Ok(Json(user))
}

pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
) -> Result<Json<QueryUser>, ApiError> {
    let user = sqlx::query_as!(
        QueryUser,
        r#"
            select id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, username
            from users
            where username = $1
        "#,
        username
    )
    .fetch_one(&pool)
    .await?;

    println!("{:?}", user);

    Ok(Json(user))
}

pub async fn get_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<QueryUser>>, ApiError> {
    let users = sqlx::query_as!(
        QueryUser,
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

pub async fn verify_pwd(password: String) -> Result<String, ApiError> {
    let pwd = password::hash(password.clone()).await?;
    let res = password::verify(password, pwd).await?;

    Ok(res.to_string())
}
