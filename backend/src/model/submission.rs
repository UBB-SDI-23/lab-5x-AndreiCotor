use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::{submissions};
use crate::model::problem::Problem;
use crate::model::user::User;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Problem))]
pub struct Submission {
    pub id: i32,
    pub user_id: i32,
    pub problem_id: i32,
    pub source_code: String,
    pub score: i32,
    pub language: String,
}

impl Submission {
    pub fn is_valid(&self) -> bool {
        0 <= self.score && self.score <= 100
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[diesel(table_name = submissions)]
pub struct NewSubmission {
    pub user_id: i32,
    pub problem_id: i32,
    pub source_code: String,
    pub score: i32,
    pub language: String,
}

impl NewSubmission {
    pub fn is_valid(&self) -> bool {
        0 <= self.score && self.score <= 100
    }
}