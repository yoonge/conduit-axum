use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use super::schema::users;

#[derive(Debug, Queryable, Selectable, Serialize)]
pub struct User {
    pub id: u64,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub birthday: Option<String>,
    pub email: String,
    pub favorite: Vec<u64>,
    pub gender: Option<u64>,
    pub nickname: Option<String>,
    pub password: String,
    pub phone: Option<u64>,
    pub position: Option<String>,
    pub username: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub username: &'a str,
}
