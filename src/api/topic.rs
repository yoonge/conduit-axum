use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{AppError, AppResponse};
use crate::db::{NewTopic, Topic};

pub async fn create_topic(
    State(pool): State<Pool<Postgres>>,
    Json(new_topic): Json<NewTopic>,
) -> Result<Json<AppResponse<Topic>>, AppError> {
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

    let res = AppResponse {
        code: StatusCode::CREATED.into(),
        data: { topic },
        msg: "Topic create succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn get_topic(
    State(pool): State<Pool<Postgres>>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<AppResponse<Topic>>, AppError> {
    let topic: Topic = sqlx::query_as(
        r#"
            select _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from (
                    select _id, avatar, bio, birthday, to_char(create_at, 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, nickname, phone, position, username
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

    let res = AppResponse {
        code: StatusCode::OK.into(),
        data: { topic },
        msg: "User query succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}

pub async fn get_topics(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<AppResponse<Vec<Topic>>>, AppError> {
    let topics: Vec<Topic> = sqlx::query_as(
        r#"
            select _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from (
                    select _id, avatar, bio, birthday, to_char(create_at, 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, nickname, phone, position, username
                    from users
                    where _id = t.user_id
                ) u
            ) as user
            from topics t
        "#
    )
    .fetch_all(&pool)
    .await?;

    let res = AppResponse {
        code: StatusCode::OK.into(),
        data: { topics },
        msg: "Topics query succeed.".to_string(),
    };

    println!("\n{:?}\n", res);

    Ok(Json(res))
}
