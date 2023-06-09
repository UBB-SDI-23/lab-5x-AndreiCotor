use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::contest;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Identifiable)]
#[diesel(table_name = contest)]
pub struct Contest {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub uid: i32
}

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Identifiable)]
#[diesel(table_name = contest)]
pub struct UpdContest {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = contest)]
pub struct NewContest {
    pub name: String,
    pub description: Option<String>,
    pub uid: Option<i32>
}