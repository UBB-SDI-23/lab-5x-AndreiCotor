use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::chat;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = chat)]
pub struct Chat {
    pub nickname: String,
    pub message: String,
    pub uid: Option<i32>
}