use crate::model::problem::Problem;
use crate::model::user::User;
use serde::{Serialize};
use crate::model::submission::Submission;

#[derive(Serialize, Clone, Debug)]
pub struct SubmissionDTO {
    #[serde(flatten)]
    pub submission: Submission,
    pub user: User,
    pub problem: Problem,
}

#[derive(Serialize, Clone, Debug)]
pub struct SubmissionReportDTO {
    #[serde(flatten)]
    pub submission: Submission,
    pub number_of_other_submissions: i32
}