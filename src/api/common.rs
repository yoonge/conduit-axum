use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{utils::topic_fmt, AppError, PAGE_SIZE};
use crate::db::{Topic, User};

pub async fn query_user(pool: &Pool<Postgres>, user_id: Uuid) -> Result<User, AppError> {
    let user: User = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, nickname, phone, position, update_at, username
            from users
            where _id = $1
        "#
    ).bind(&user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_topics(
    pool: &Pool<Postgres>,
    page: i32,
    username: String,
) -> Result<(Vec<Topic>, i64), AppError> {
    let offset = (page - 1) * PAGE_SIZE;

    let topics: Vec<Topic> = sqlx::query_as(
        r#"
            with u as
            (
                select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, nickname, phone, position, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                from users
                where username = $1
            )
            select _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from u
            ) as user
            from topics t
            where t.user_id = (select _id from u)
            order by update_at desc
            limit $2 offset $3
        "#
    )
    .bind(&username)
    .bind(PAGE_SIZE)
    .bind(&offset)
    .fetch_all(pool)
    .await?;

    let total: i64 = sqlx::query_scalar(
        r#"
            with u as (
                select _id from users where username = $1
            )
            select count(*) from topics t
            where t.user_id = (select _id from u)
        "#,
    )
    .bind(&username)
    .fetch_one(pool)
    .await?;

    let topics = topic_fmt::format(topics)?;

    Ok((topics, total))
}

pub async fn get_user_favorites(
    pool: &Pool<Postgres>,
    page: i32,
    username: String,
) -> Result<(Vec<Topic>, i64), AppError> {
    let offset = (page - 1) * PAGE_SIZE;

    let topics: Vec<Topic> = sqlx::query_as(
        r#"
            with u as
            (
                select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, nickname, phone, position, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                from users
                where username = $1
            )
            select _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u) from u
            ) as user
            from topics t
            where t._id = any((select favorite from u)::uuid[])
            order by update_at desc
            limit $2 offset $3
        "#
    )
    .bind(&username)
    .bind(PAGE_SIZE)
    .bind(&offset)
    .fetch_all(pool)
    .await?;

    let total: i32 = sqlx::query_scalar(
        r#"
            select array_length(favorite, 1) from users where username = $1
        "#,
    )
    .bind(&username)
    .fetch_one(pool)
    .await?;

    let topics = topic_fmt::format(topics)?;

    Ok((topics, total as i64))
}
