use crate::model::problem::Problem;
use serde::{Serialize};
use crate::model::submission::Submission;

#[derive(Serialize, Clone, Debug)]
pub struct ProblemDTO {
    #[serde(flatten)]
    pub problem: Problem,
    pub submissions: Vec<Submission>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProblemStatisticsDTO {
    #[serde(flatten)]
    pub problem: Problem,
    pub cnt: i32
}

#[derive(Serialize, Clone, Debug)]
pub struct ProblemByOtherSolvedProblemsDTO {
    #[serde(flatten)]
    pub problem: Problem,
    pub number_of_other_solved_problems_by_solvers: i32
}

#[derive(Serialize, Clone, Debug)]
pub struct ProblemWithCreatorDTO {
    #[serde(flatten)]
    pub problem: Problem,
    pub cnt: i32,
    pub creator: String
}
