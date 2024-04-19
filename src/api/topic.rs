use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::api::AppError;
use crate::db::Topic;

#[derive(Deserialize)]
pub struct NewTopic {
    pub content: String,
    pub title: String,
    pub user_id: Uuid,
}

pub async fn create_topic(
    State(pool): State<Pool<Postgres>>,
    Json(new_topic): Json<NewTopic>,
) -> Result<Json<Topic>, AppError> {
    let topic: Topic = sqlx::query_as(
        r#"
            insert into topics (content, title, user_id)
            values ($1, $2, $3)
            returning *
        "#,
    )
    .bind(&new_topic.content)
    .bind(&new_topic.title)
    .bind(&new_topic.user_id)
    .fetch_one(&pool)
    .await?;

    println!("{:?}", topic);

    Ok(Json(topic))
}

pub async fn get_topic(
    State(pool): State<Pool<Postgres>>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<Topic>, AppError> {
    let topic: Topic = sqlx::query_as(
        r#"
            select comments, content, create_at, favorite, _id, tags, title, update_at, user_id
            from topics
            where _id = $1
        "#,
    )
    .bind(topic_id)
    .fetch_one(&pool)
    .await?;

    println!("{:?}", topic);

    Ok(Json(topic))
}
