use crate::model::user::User;
use serde::{Serialize};
use crate::model::submission::Submission;

#[derive(Serialize, Clone, Debug)]
pub struct UserDTO {
    #[serde(flatten)]
    pub user: User,
    pub submissions: Vec<Submission>,
}

#[derive(Serialize, Clone, Debug)]
pub struct UserReportDTO {
    #[serde(flatten)]
    pub user: User,
    pub participations: i32
}

#[derive(Serialize, Clone, Debug)]
pub struct UserSubmissionsDTO {
    #[serde(flatten)]
    pub user: User,
    pub cnt: i32
}

#[derive(Serialize, Clone, Debug)]
pub struct UserPageDTO {
    #[serde(flatten)]
    pub user: User,
    pub username: String,
    pub problems_proposed: i32,
    pub contests_created: i32,
    pub submissions_sent: i32,
    pub participations: i32
}