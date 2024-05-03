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
    common, tag,
    utils::{jwt::Claims, topic_fmt},
    AppError, PAGE_SIZE,
};
use crate::db::{Comment, NewComment, NewTopic, Topic, TopicPayload};

pub async fn create_topic(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<NewTopic>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let mut tags = payload.tags.clone();
    tags = tags
        .iter()
        .map(|tag| tag.to_lowercase())
        .collect::<Vec<String>>();
    tags.sort();
    println!("\nSorted tags: {:?}\n", tags);

    let topic: Topic = sqlx::query_as(
        r#"
            insert into topics (content, tags, title, user_id)
            values ($1, $2, $3, $4)
            returning _id, comments, content, create_at, favorite, tags, title, update_at, user_id
        "#,
    )
    .bind(&payload.content)
    .bind(&tags)
    .bind(&payload.title)
    .bind(&payload.user_id)
    .fetch_one(&pool)
    .await?;

    tag::update_tags(pool, tags, vec![], topic._id).await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topic create succeed."));
    res.insert("topic".to_string(), json!(&topic));

    Ok(Json(json!(res)))
}

pub async fn get_topic(
    State(pool): State<Pool<Postgres>>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    let topic = common::query_topic(&pool, topic_id).await?;
    let mut topic = json!(&topic);
    topic["comments"] = topic["comments_arr"].clone();
    topic["comments_arr"].take();

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topic query succeed."));
    res.insert("topic".to_string(), topic);

    Ok(Json(json!(res)))
}

pub async fn get_update_topic(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let topic = common::query_topic(&pool, topic_id).await?;
    let mut topic = json!(&topic);
    topic["comments"] = topic["comments_arr"].clone();
    topic["comments_arr"].take();
    let user = topic["user"].clone();

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topic delete succeed."));
    res.insert("topic".to_string(), topic);
    res.insert("user".to_string(), user);

    Ok(Json(json!(res)))
}

pub async fn topic_update(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<TopicPayload>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let mut tags = payload.tags.clone();
    tags = tags
        .iter()
        .map(|tag| tag.to_lowercase())
        .collect::<Vec<String>>();
    tags.sort();
    println!("\nSorted tags: {:?}\n", tags);

    let topic: Topic = sqlx::query_as(
        r#"
            with u as (
                select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, job, nickname, phone, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                from users
                where _id = $1
            )
            update topics
            set content = $2, tags = $3, title = $4
            where _id = $5
            returning _id, comments, (
                select json_agg(cs) from (
                    select _id, content, create_at, topic, user_id
                    from comments
                    where topic = $5
                    order by create_at desc
                ) as cs
            ) as comments_arr, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from u
            ) as user
        "#,
    )
    .bind(&payload.user_id)
    .bind(&payload.content)
    .bind(&tags)
    .bind(&payload.title)
    .bind(&payload._id)
    .fetch_one(&pool)
    .await?;

    tag::update_tags(pool, tags, payload.tags_removed, payload._id).await?;

    let mut topic = json!(&topic);
    topic["comments"] = topic["comments_arr"].clone();
    topic["comments_arr"].take();

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topic update succeed."));
    res.insert("topic".to_string(), topic);

    Ok(Json(json!(res)))
}

pub async fn topic_comment(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<NewComment>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let topic: Topic = sqlx::query_as(
        r#"
            with
                u as (
                    select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, job, nickname, phone, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                    from users
                    where _id = $3
                ),
                c as (
                    insert into comments (content, topic, user_id)
                    values ($1, $2, $3)
                    returning _id, content, create_at, topic, user_id
                )
            update topics t
            set comments = array_append(t.comments, c._id)
            from c
            where t._id = $2
            returning t._id, t.comments, t.content, t.create_at, t.favorite, t.tags, t.title, t.update_at, t.user_id, (
                select row_to_json(u) from u
            ) as user
        "#
    )
    .bind(&payload.content)
    .bind(&payload.topic)
    .bind(&payload.user_id)
    .fetch_one(&pool)
    .await?;

    let comments: Vec<Comment> = sqlx::query_as(
        r#"
            select _id, content, create_at, topic, user_id
            from comments
            where topic = $1
            order by create_at desc
        "#,
    )
    .bind(&payload.topic)
    .fetch_all(&pool)
    .await?;

    let mut topic = json!(&topic);
    topic["comments"] = json!(&comments);

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topic comment succeed."));
    res.insert("updatedTopic".to_string(), topic);

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

    Ok(Json(json!(res)))
}

pub async fn get_user_profile(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);
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

    Ok(Json(json!(res)))
}

pub async fn get_user_favorites(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);
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

    Ok(Json(json!(res)))
}
