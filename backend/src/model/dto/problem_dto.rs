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
    pub success_rate: Option<f32>
}

impl ProblemStatisticsDTO {
    pub fn new(problem: Problem, submission_list: &Vec<Submission>) -> Self {
        if submission_list.len() == 0 {
            return ProblemStatisticsDTO {
                problem,
                success_rate: None
            }
        }

        let successful_attempts = submission_list.iter().filter(|&x| x.score == 100).count();
        ProblemStatisticsDTO {
            problem,
            success_rate: Some((successful_attempts as f32) / (submission_list.len() as f32))
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ProblemByOtherSolvedProblemsDTO {
    #[serde(flatten)]
    pub problem: Problem,
    pub number_of_other_solved_problems_by_solvers: i32
}
