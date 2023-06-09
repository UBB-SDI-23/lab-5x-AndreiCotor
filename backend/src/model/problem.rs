use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::problems;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Identifiable)]
pub struct Problem {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) contest: String,
    pub(crate) statement: String,
    pub(crate) rating: i32,
    pub uid: i32
}

impl Problem {
    pub fn is_valid(&self) -> bool {
        0 <= self.rating && self.rating <= 5
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = problems)]
pub struct NewProblem {
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) contest: String,
    pub(crate) statement: String,
    pub(crate) rating: i32,
    pub uid: Option<i32>
}

impl NewProblem {
    pub fn is_valid(&self) -> bool {
        0 <= self.rating && self.rating <= 5 && !self.name.is_empty() && self.name.len() >= 3
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable, AsChangeset)]
#[diesel(table_name = problems)]
pub struct UpdProblem {
    pub id: i32,
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) contest: String,
    pub(crate) statement: String,
    pub(crate) rating: i32
}

impl UpdProblem {
    pub fn is_valid(&self) -> bool {
        0 <= self.rating && self.rating <= 5 && !self.name.is_empty() && self.name.len() >= 3
    }
}