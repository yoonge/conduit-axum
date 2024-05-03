use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::api::utils::date_fmt;

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = std::env::var("DATABASE_URL").expect("`DATABASE_URL` must be set.");
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

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub _id: Uuid,
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    #[serde(with = "date_fmt")]
    pub create_at: DateTime<Local>,
    pub email: String,
    pub favorite: Vec<Uuid>,
    // 1: male, 0: female, -1: secret
    pub gender: i16,
    pub job: String,
    pub nickname: String,
    #[sqlx(default)]
    pub password: Option<String>,
    pub phone: String,
    #[serde(with = "date_fmt")]
    pub update_at: DateTime<Local>,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserPayload {
    pub _id: Uuid,
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    pub email: String,
    pub gender: i16,
    pub job: String,
    pub nickname: String,
    pub password: Option<String>,
    pub phone: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct NewTopic {
    pub content: String,
    pub tags: Vec<String>,
    pub title: String,
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct Topic {
    pub _id: Uuid,
    pub comments: Vec<Uuid>,
    #[sqlx(default)]
    pub comments_arr: Option<Value>,
    pub content: String,
    #[sqlx(default)]
    pub content_clip: Option<String>,
    #[serde(with = "date_fmt")]
    pub create_at: DateTime<Local>,
    pub favorite: i32,
    // #[serde(bound = "T: PartialEq + Eq + PartialOrd + Ord")]
    pub tags: Vec<String>,
    pub title: String,
    #[sqlx(default)]
    pub title_clip: Option<String>,
    #[serde(with = "date_fmt")]
    pub update_at: DateTime<Local>,
    #[sqlx(default)]
    pub update_at_str: Option<String>,
    pub user_id: Uuid,
    #[sqlx(default)]
    pub user: Option<Value>
}

#[derive(Deserialize)]
pub struct TopicPayload {
    pub _id: Uuid,
    pub content: String,
    pub tags: Vec<String>,
    pub tags_removed: Vec<String>,
    pub title: String,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct FavorPayload {
    pub topic_id: Uuid,
    // user_id: Option<Uuid>,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct Comment {
    pub _id: Uuid,
    pub content: String,
    #[serde(with = "date_fmt")]
    pub create_at: DateTime<Local>,
    pub topic: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct NewComment {
    pub content: String,
    pub topic: Uuid,
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct Tag {
    pub _id: Uuid,
    #[serde(with = "date_fmt")]
    pub create_at: DateTime<Local>,
    pub tag: String,
    pub topics: Vec<Uuid>,
}
