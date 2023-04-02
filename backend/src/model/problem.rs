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
    pub(crate) rating: i32
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
    pub(crate) rating: i32
}

impl NewProblem {
    pub fn is_valid(&self) -> bool {
        0 <= self.rating && self.rating <= 5
    }
}