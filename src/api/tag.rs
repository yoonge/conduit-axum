use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Map, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db::{Tag, Topic};

use super::{utils::topic_fmt, AppError, PAGE_SIZE};

pub async fn get_tags(State(pool): State<Pool<Postgres>>) -> Result<Json<Value>, AppError> {
    let tags: Vec<Tag> = sqlx::query_as(
        r#"
            select _id, create_at, tag, topics, array_length(topics, 1) as count
            from tags
            order by create_at desc
        "#
    )
    .fetch_all(&pool)
    .await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Tags query succeed."));
    res.insert("tags".to_string(), json!(&tags));

    Ok(Json(json!(res)))
}

pub async fn get_topics_by_tag(
    State(pool): State<Pool<Postgres>>,
    Path(tag): Path<String>,
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
            where $1 = any(t.tags::text[])
            order by update_at desc
            limit $2 offset $3
        "#
    )
    .bind(&tag)
    .bind(PAGE_SIZE)
    .bind(&offset)
    .fetch_all(&pool)
    .await?;

    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*)
            from topics t
            where $1 = any(t.tags::text[])
        "#,
    )
    .bind(&tag)
    .fetch_optional(&pool)
    .await?
    .unwrap_or(0);

    let topics = topic_fmt::format(topics)?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Topics query succeed."));
    res.insert("page".to_string(), json!(&page));
    res.insert("topics".to_string(), json!(&topics));
    res.insert("total".to_string(), json!(&total));

    Ok(Json(json!(res)))
}

pub async fn update_tags(
    pool: Pool<Postgres>,
    tags: Vec<String>,
    tags_removed: Vec<String>,
    topic_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
            select update_tags($1, $2)
        "#
    )
    .bind(&tags)
    .bind(&topic_id)
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
            select remove_tags($1, $2)
        "#
    )
    .bind(&tags_removed)
    .bind(&topic_id)
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
            delete from tags
            where topics = array[]::uuid[]
        "#
    )
    .execute(&pool)
    .await?;

    Ok(())
}
