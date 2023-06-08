use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Identifiable, Insertable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub school: String,
    pub bio: String,
    pub teacher: String
}

impl User {
    pub(crate) fn from_id(id: i32, username: String) -> Self {
        User {
            id,
            first_name: "".to_string(),
            last_name: username,
            school: "".to_string(),
            bio: "".to_string(),
            teacher: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub school: String,
    pub bio: String,
    pub teacher: String
}