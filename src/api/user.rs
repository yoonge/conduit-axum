use serde::Deserialize;
use axum::{extract::State, Json};
use sqlx::{postgres::PgRow, Pool, Postgres};

use super::ApiError;
// use conduit_axum::db::User;

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<PgRow>, ApiError> {
    let res = sqlx::query::<Postgres>("insert into users (email, password, username) values ($1, $2, $3)")
        .bind(&new_user.email)
        .bind(&new_user.password)
        .bind(&new_user.username)
        .execute(&pool)
        .await?;

    let user = sqlx::query("select * from users where email = ?")
        .bind(&new_user.email)
        .fetch_one(&pool)
        .await?;

    Ok(Json(user))
}
