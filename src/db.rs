use std::env;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Pool, Postgres};
use time::PrimitiveDateTime;

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set.");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database.");
    pool
}

#[derive(Debug, Deserialize, FromRow, PartialEq, Serialize)]
pub struct User {
    pub id: i32,
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    pub created_at: PrimitiveDateTime,
    pub email: String,
    pub favorite: Vec<i32>,
    // 1: male, 0: female, -1: secret
    pub gender: i16,
    pub nickname: String,
    pub password: String,
    pub phone: String,
    pub position: String,
    pub username: String,
}
