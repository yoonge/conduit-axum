use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{utils::password, AppError, AppResponse};
use crate::db::{NewUser, User};

pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<AppResponse<User>>, AppError> {
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

    let res = AppResponse::new(
        StatusCode::CREATED.into(),
        user,
        "User create succeed.".to_string(),
    );

    println!("{:?}", res);

    Ok(Json(res))
}

pub async fn _query_user(pool: Pool<Postgres>, user_id: Uuid) -> Result<User, AppError> {
    let user: User = sqlx::query_as(
        r#"
            select avatar, bio, birthday, create_at, email, favorite, gender, _id, nickname, phone, position, username
            from users
            where _id = $1
        "#
    ).bind(&user_id)
    .fetch_one(&pool)
    .await?;

    Ok(user)
}

pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(username): Path<String>,
) -> Result<Json<AppResponse<User>>, AppError> {
    let user: User = sqlx::query_as(
        r#"
            select avatar, bio, birthday, create_at, email, favorite, gender, _id, nickname, phone, position, username
            from users
            where username = $1
        "#
    ).bind(username)
    .fetch_one(&pool)
    .await?;

    let res = AppResponse::new(
        StatusCode::OK.into(),
        user,
        "User query succeed.".to_string(),
    );

    println!("{:?}", res);

    Ok(Json(res))
}

pub async fn get_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<AppResponse<Vec<User>>>, AppError> {
    let users: Vec<User> = sqlx::query_as(
        r#"
            select avatar, bio, birthday, create_at, email, favorite, gender, _id, nickname, phone, position, username
            from users
        "#
    )
    .fetch_all(&pool)
    .await?;

    let res = AppResponse::new(
        StatusCode::OK.into(),
        users,
        "Users query succeed.".to_string(),
    );

    println!("{:?}", res);

    Ok(Json(res))
}

pub async fn verify_pwd(password: String) -> Result<String, AppError> {
    let pwd = password::hash(password.clone()).await?;
    let res = password::verify(password, pwd).await?;

    Ok(res.to_string())
}
