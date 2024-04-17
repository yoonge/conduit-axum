use std::env;
use serde::Serialize;
use sqlx::{FromRow, PgPool, Pool, Postgres};

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set.");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database.");
    pool
}

#[derive(Debug, FromRow, PartialEq, Serialize)]
pub struct User {
    pub id: u64,
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    pub created_at: String,
    pub email: String,
    pub favorite: Vec<u64>,
    // 1: male, 0: female, -1: secret
    pub gender: i8,
    pub nickname: String,
    pub password: String,
    pub phone: String,
    pub position: String,
    pub username: String,
}
