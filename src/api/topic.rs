use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Map, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{
    common,
    utils::{jwt::Claims, topic_fmt},
    AppError, PAGE_SIZE,
};
use crate::db::{NewTopic, Topic};

pub async fn create_topic(
    State(pool): State<Pool<Postgres>>,
    Json(new_topic): Json<NewTopic>,
) -> Result<Json<Value>, AppError> {
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

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topic create succeed."));
    res.insert("topic".to_string(), json!(&topic));

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
                    select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, job, nickname, phone, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
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
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topic query succeed."));
    res.insert("topic".to_string(), json!(&topic));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_topics(
    State(pool): State<Pool<Postgres>>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\nQuery Args: {:?}\n", args);
    let page = args
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()?;
    let offset = (page - 1) * PAGE_SIZE;

    let topics: Vec<Topic> = sqlx::query_as(
        r#"
            select _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from (
                    select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, job, nickname, phone, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                    from users
                    where _id = t.user_id
                ) u
            ) as user
            from topics t
            order by update_at desc
            limit $1 offset $2
        "#
    )
    .bind(PAGE_SIZE)
    .bind(&offset)
    .fetch_all(&pool)
    .await?;

    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*) from topics
        "#,
    )
    .fetch_one(&pool)
    .await?;

    let topics = topic_fmt::format(topics)?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topics query succeed."));
    res.insert("page".to_string(), json!(&page));
    res.insert("topics".to_string(), json!(&topics));
    res.insert("total".to_string(), json!(&total));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_user_profile(
    _claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", _claims);
    println!("\nQuery Args: {:?}\n", args);
    let page = args
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()?;

    let (topics, total) = common::get_user_topics(&pool, page, username).await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User's profile query succeed."));
    res.insert("page".to_string(), json!(&page));
    res.insert("topics".to_string(), json!(&topics));
    res.insert("total".to_string(), json!(&total));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_user_favorites(
    _claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", _claims);
    println!("\nQuery Args: {:?}\n", args);
    let page = args
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()?;

    let (topics, total) = common::get_user_favorites(&pool, page, username).await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert(
        "msg".to_string(),
        json!("User's favorite topics query succeed."),
    );
    res.insert("page".to_string(), json!(&page));
    res.insert("topics".to_string(), json!(&topics));
    res.insert("total".to_string(), json!(&total));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}
