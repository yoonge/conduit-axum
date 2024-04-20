use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Pool, Postgres, types::JsonValue};
use std::env;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::api::utils::date_formatter;

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set.");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database.");
    pool
}

#[derive(Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    #[serde(with = "date_formatter")]
    pub create_at: OffsetDateTime,
    pub email: String,
    pub favorite: Vec<Uuid>,
    // 1: male, 0: female, -1: secret
    pub gender: i16,
    pub _id: Uuid,
    pub nickname: String,
    #[sqlx(default)]
    pub password: Option<String>,
    pub phone: String,
    pub position: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct NewTopic {
    pub content: String,
    pub title: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Topic {
    pub comments: Vec<Uuid>,
    pub content: String,
    #[serde(with = "date_formatter")]
    pub create_at: OffsetDateTime,
    pub favorite: i32,
    pub _id: Uuid,
    pub tags: Vec<String>,
    pub title: String,
    #[serde(with = "date_formatter")]
    pub update_at: OffsetDateTime,
    pub user_id: Uuid,
    #[sqlx(default)]
    pub user: Option<JsonValue>
}
