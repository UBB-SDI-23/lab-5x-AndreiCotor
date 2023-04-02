use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Identifiable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub school: String,
    pub bio: String,
    pub teacher: String
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