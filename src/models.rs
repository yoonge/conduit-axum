use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use super::schema::users;

#[derive(Debug, Queryable, Selectable, Serialize)]
pub struct User {
    pub id: u64,
    pub avatar: String,
    pub bio: String,
    pub birthday: String,
    pub created_at: String,
    pub email: String,
    pub favorite: Vec<u64>,
    // 1: male, 0: female, -1: secret
    pub gender: i8,
    pub nickname: String,
    pub password: String,
    pub phone: u64,
    pub position: String,
    pub username: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub username: &'a str,
}
