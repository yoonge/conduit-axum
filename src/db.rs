use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Pool, Postgres};
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

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    #[serde(with = "date_formatter")]
    pub create_at: OffsetDateTime,
    pub email: String,
    pub favorite: Vec<Uuid>,
    // 1: male, 0: female, -1: secret
    pub gender: i16,
    pub nickname: String,
    #[sqlx(default)]
    pub password: Option<String>,
    pub phone: String,
    pub position: String,
    pub username: String,
}
