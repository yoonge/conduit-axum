use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use std::env;
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set.");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database.");
    pool
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub password: String,
    pub phone: String,
    pub position: String,
    pub username: String,
}

pub mod date_formatter {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use time::{macros::format_description, OffsetDateTime};

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let formatted = date.format(&format).unwrap();
        serializer.serialize_str(&formatted)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        let s = String::deserialize(deserializer)?;
        OffsetDateTime::parse(&s, &format).map_err(D::Error::custom)
    }
}
