use std::collections::HashMap;

use anyhow::anyhow;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{
    common,
    utils::{
        jwt::{AuthError, AuthPayload, Claims, KEYS},
        password,
    },
    AppError, PAGE_SIZE,
};
use crate::{
    api::utils::topic_fmt,
    db::{NewUser, Topic, User},
};

pub async fn login(
    State(_pool): State<Pool<Postgres>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>, AppError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::Auth(AuthError::MissingCredentials));
    }

    let hashed_password = password::hash(payload.password).await?;
    let user: User = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, job, nickname, phone, update_at, username
            from users
            where email = $1 and password = $2
        "#
    )
    .bind(&payload.email)
    .bind(&hashed_password)
    .fetch_one(&_pool)
    .await
    .map_err(|_| AppError::Auth(AuthError::InvalidCredentials))?;

    let user_clone = user.clone();
    let claims = Claims::new(user_clone._id, user_clone.nickname, user_clone.username);
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User login succeed."));
    res.insert("token".to_string(), json!(token));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<Value>, AppError> {
    let count: i64 = sqlx::query_scalar(
        r#"
            select count(*)
            from users
            where email = $1 or username = $2
        "#,
    )
    .bind(&new_user.email)
    .bind(&new_user.username)
    .fetch_one(&pool)
    .await?;

    if count > 0 {
        return Err(AppError::Duplicate(anyhow!(
            "Email or username already exsists."
        )));
    }

    let hashed_password = password::hash(new_user.password).await?;
    let user: User = sqlx::query_as(
        r#"
            insert into users (email, password, username)
            values ($1, $2, $3)
            returning *
        "#,
    )
    .bind(&new_user.email)
    .bind(&hashed_password)
    .bind(&new_user.username)
    .fetch_one(&pool)
    .await?;

    let user_clone = user.clone();
    let claims = Claims::new(user_clone._id, user_clone.nickname, user_clone.username);
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User register succeed."));
    res.insert("token".to_string(), json!(token));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_user(
    _claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", _claims);

    let user: User = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, job, nickname, phone, update_at, username
            from users
            where username = $1
        "#
    ).bind(username)
    .fetch_one(&pool)
    .await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User query succeed."));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_users(
    _claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", _claims);
    println!("\nQuery Args: {:?}\n", args);
    let page = args
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()?;
    let offset = (page - 1) * PAGE_SIZE;

    let users: Vec<User> = sqlx::query_as(
        r#"
            select _id, avatar, bio, birthday, create_at, email, favorite, gender, job, nickname, phone, update_at, username
            from users
            order by create_at desc
            limit $1 offset $2
        "#
    )
    .bind(PAGE_SIZE)
    .bind(&offset)
    .fetch_all(&pool)
    .await?;

    let total: i64 = sqlx::query_scalar(
        r#"
            select count(*) from users
        "#,
    )
    .fetch_one(&pool)
    .await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("Users query succeed."));
    res.insert("page".to_string(), json!(&page));
    res.insert("total".to_string(), json!(&total));
    res.insert("users".to_string(), json!(&users));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_my_settings(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let user = common::query_user(&pool, claims.cuid).await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User settings query succeed."));
    res.insert("user".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn update_my_settings(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let user: User = sqlx::query_as(
        r#"
            update users
            set
                avatar = case when $1 is not null then $1 else avatar end,
                bio = case when $2 is not null then $2 else bio end,
                birthday = case when $3 is not null then $3 else birthday end,
                email = case when $4 is not null then $4 else email end,
                gender = case when $5 is not null then $5 else gender end,
                nickname = case when $6 is not null then $6 else nickname end,
                job = case when $7 is not null then $7 else job end,
                password = case when $8 != '' and $8 is not null then $1 else password end,
                phone = case when $9 is not null then $9 else phone end,
                username = case when $10 is not null then $10 else username end
            where _id = $11
            returning *
        "#,
    )
    .bind(&payload.avatar)
    .bind(&payload.bio)
    .bind(&payload.birthday)
    .bind(&payload.email)
    .bind(&payload.gender)
    .bind(&payload.nickname)
    .bind(&payload.job)
    .bind(&payload.password)
    .bind(&payload.phone)
    .bind(&payload.username)
    .bind(&payload._id)
    .fetch_one(&pool)
    .await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User settings update succeed."));
    res.insert("updatedUser".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_my_topics(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);
    println!("\nQuery Args: {:?}\n", args);
    let page = args
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()?;

    let (topics, total) = common::get_user_topics(&pool, page, claims.username).await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User's own topics query succeed."));
    res.insert("page".to_string(), json!(&page));
    res.insert("topics".to_string(), json!(&topics));
    res.insert("total".to_string(), json!(&total));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn get_my_favorites(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Query(args): Query<HashMap<String, String>>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);
    println!("\nQuery Args: {:?}\n", args);
    let page = args
        .get("page")
        .unwrap_or(&"1".to_string())
        .parse::<i32>()?;

    let (topics, total) = common::get_user_favorites(&pool, page, claims.username).await?;

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User's favorite topics query succeed."));
    res.insert("page".to_string(), json!(&page));
    res.insert("topics".to_string(), json!(&topics));
    res.insert("total".to_string(), json!(&total));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

pub async fn favor(
    claims: Claims,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<FavorPayload>,
) -> Result<Json<Value>, AppError> {
    println!("\n{:?}\n", claims);

    let topic: Topic = sqlx::query_as(
        r#"
            with u as (
                select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, job, nickname, phone, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                from users
                where _id = $1
            )
            update topics t
            set favorite =
                case
                    when $2 = any((select favorite from u)::uuid[]) then
                        favorite - 1
                    else
                        favorite + 1
                end
            where _id = $2
            returning _id, comments, content, create_at, favorite, tags, title, update_at, user_id, (
                select row_to_json(u1) from (
                    select _id, avatar, bio, birthday, to_char(create_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as create_at, email, favorite, gender, job, nickname, phone, to_char(update_at + interval '8 hours', 'YYYY-MM-DD HH24:MI:SS') as update_at, username
                    from users
                    where _id = t.user_id
                ) u1
            ) as user
        "#
    )
    .bind(&claims.cuid)
    .bind(&payload.topic_id)
    .fetch_one(&pool)
    .await?;

    let topics = topic_fmt::format(vec![topic])?;

    let user: User = sqlx::query_as(
        r#"
            with u as (
                select favorite
                from users
                where _id = $1
            )
            update users
            set favorite =
                case
                    when $2 = any((select favorite from u)::uuid[]) then
                        array_remove((select favorite from u)::uuid[], $2)
                    else
                        array_append((select favorite from u)::uuid[], $2)
                end
            where _id = $1
            returning _id, avatar, bio, birthday, create_at, email, favorite, gender, job, nickname, phone, update_at, username
        "#
    )
    .bind(&claims.cuid)
    .bind(&payload.topic_id)
    .fetch_one(&pool)
    .await?;

    let mut topic = topics[0].clone();
    if topic.user_id == claims.cuid {
        topic.user = Some(json!(&user));
    }

    let mut res = Map::new();
    res.insert("code".to_string(), json!(StatusCode::OK.as_u16()));
    res.insert("msg".to_string(), json!("User favor / disfavor succeed."));
    res.insert("updatedTopic".to_string(), json!(&topic));
    res.insert("updatedUser".to_string(), json!(&user));

    println!("\n{:?}\n", res);

    Ok(Json(json!(res)))
}

#[derive(Deserialize)]
pub struct FavorPayload {
    topic_id: Uuid,
    // user_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UserPayload {
    _id: Uuid,
    avatar: String,
    bio: String,
    birthday: String,
    email: String,
    gender: i16,
    job: String,
    nickname: String,
    password: String,
    phone: String,
    username: String,
}
