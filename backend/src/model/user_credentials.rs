use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::usercredentials;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Identifiable)]
#[diesel(table_name = usercredentials)]
pub struct UserCredentials {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub confirmed: bool,
    pub created: NaiveDateTime,
    pub uuid: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = usercredentials)]
pub struct NewUserCredentials {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = usercredentials)]
pub struct InsertableUserCredentials {
    pub username: String,
    pub password: String,
    pub uuid: String
}