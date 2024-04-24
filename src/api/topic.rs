use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Map, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::AppError;
use crate::{
    api::utils::topic_fmt,
    db::{NewTopic, Topic},
};

pub async fn create_topic(
    State(pool): State<Pool<Postgres>>,
    Json(new_topic): Json<NewTopic>,
) -> Result<Json<Value>, AppError> {
    let topic: Topic = sqlx::query_as(
        r#"
            insert into topics (content, title, user_id)
            values ($1, $2, $3)
            returning _id, comments, content, create_at, favorite, tags, title, update_at, user_id
        "#,
    )
    .bind(&new_topic.content)
    .bind(&new_topic.title)
    .bind(&new_topic.user_id)
    .fetch_one(&pool)
    .await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_str()));
    res.insert("topic".to_string(), json!(&topic));
    res.insert("msg".to_string(), json!("Topic create succeed."));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_topic(
    State(pool): State<Pool<Postgres>>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    let topic: Topic = sqlx::query_as(
        r#"
            select _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from (
                    select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, nickname, phone, position, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                    from users
                    where _id = t.user_id
                ) u
            ) as user
            from topics t
            where _id = $1
        "#,
    )
    .bind(&topic_id)
    .fetch_one(&pool)
    .await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_str()));
    res.insert("topic".to_string(), json!(&topic));
    res.insert("msg".to_string(), json!("Topic query succeed."));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_topics(State(pool): State<Pool<Postgres>>) -> Result<Json<Value>, AppError> {
    let mut topics: Vec<Topic> = sqlx::query_as(
        r#"
            select _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from (
                    select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, nickname, phone, position, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                    from users
                    where _id = t.user_id
                ) u
            ) as user
            from topics t
            order by update_at desc
        "#
    )
    .fetch_all(&pool)
    .await?;

    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*) from topics
        "#,
    )
    .fetch_one(&pool)
    .await?;

    let topics = topic_fmt::format(&mut topics)?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_str()));
    res.insert("topics".to_string(), json!(&topics));
    res.insert("total".to_string(), json!(&total));
    res.insert("msg".to_string(), json!("Topics query succeed."));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}
