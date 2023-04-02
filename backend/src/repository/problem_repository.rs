use crate::model::problem::{NewProblem, Problem};
use crate::model::submission::Submission;
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn get_all_problems(db: &mut Mockable<DbConn>) -> Result<Vec<Problem>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_problems(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_problem_by_id(db: &mut Mockable<DbConn>, uid: i32) -> Result<Option<Problem>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problem_by_id(inner, uid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_problems_rating_larger(db: &mut Mockable<DbConn>, problem_rating: i32) -> Result<Vec<Problem>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problems_rating_larger(inner, problem_rating),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn add_problem(db: &mut Mockable<DbConn>, problem: NewProblem) {
    match db {
        Mockable::Real(inner) => real::add_problem(inner, problem),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn delete_problem(db: &mut Mockable<DbConn>, pid: i32) {
    match db {
        Mockable::Real(inner) => real::delete_problem(inner, pid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn update_problem(db: &mut Mockable<DbConn>, problem: Problem) {
    match db {
        Mockable::Real(inner) => real::update_problem(inner, problem),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_problems_with_submissions(db: &mut Mockable<DbConn>) -> Result<Vec<(Problem, Vec<Submission>)>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problems_with_submissions(inner),
        Mockable::Mock => mock::get_problems_with_submissions()
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::problem::{NewProblem, Problem};
    use crate::model::submission::Submission;
    use crate::repository::DbError;
    use crate::schema::problems::dsl::*;

    pub fn get_all_problems(db: &mut PgConnection) -> Result<Vec<Problem>, DbError> {
        let problem_list = problems.load(db)?;
        Ok(problem_list)
    }

    pub fn get_problem_by_id(db: &mut PgConnection, uid: i32) -> Result<Option<Problem>, DbError> {
        let problem = problems.filter(id.eq(uid))
            .first::<Problem>(db)
            .optional()?;

        Ok(problem)
    }

    pub fn get_problems_rating_larger(db: &mut PgConnection, problem_rating: i32) -> Result<Vec<Problem>, DbError> {
        let problem_list = problems.filter(rating.gt(problem_rating)).load(db).unwrap();
        Ok(problem_list)
    }

    pub fn add_problem(db: &mut PgConnection, problem: NewProblem) {
        diesel::insert_into(problems).values(problem).execute(db).unwrap();
    }

    pub fn delete_problem(db: &mut PgConnection, pid: i32) {
        diesel::delete(problems.filter(id.eq(pid))).execute(db).unwrap();
    }

    pub fn update_problem(db: &mut PgConnection, problem: Problem) {
        diesel::update(problems.filter(id.eq(problem.id))).set(problem).execute(db).unwrap();
    }

    pub fn get_problems_with_submissions(db: &mut PgConnection) -> Result<Vec<(Problem, Vec<Submission>)>, DbError> {
        let all_problems = problems.load(db).unwrap();
        let sumbmission_list = Submission::belonging_to(&all_problems).load(db).unwrap();

        let submissions_per_problem = sumbmission_list.grouped_by(&all_problems)
            .into_iter()
            .zip(all_problems)
            .map(|(submission, problem)| (problem, submission))
            .collect::<Vec<(Problem, Vec<Submission>)>>();

        Ok(submissions_per_problem)
    }
}

mod mock {
    use crate::model::problem::Problem;
    use crate::model::submission::Submission;
    use crate::repository::DbError;

    pub fn get_problems_with_submissions() -> Result<Vec<(Problem, Vec<Submission>)>, DbError> {
        Ok(vec![
            (Problem {
                id: 1,
                name: "1".to_string(),
                author: "1".to_string(),
                contest: "1".to_string(),
                statement: "1".to_string(),
                rating: 0,
            }, vec![Submission{
                id: 1,
                user_id: 1,
                problem_id: 1,
                source_code: "1".to_string(),
                score: 0,
                language: "1".to_string(),
            }, Submission {
                id: 2,
                user_id: 1,
                problem_id: 1,
                source_code: "2".to_string(),
                score: 50,
                language: "2".to_string(),
            }, Submission {
                id: 3,
                user_id: 1,
                problem_id: 1,
                source_code: "3".to_string(),
                score: 100,
                language: "3".to_string(),
            }]),
            (Problem {
                id: 2,
                name: "2".to_string(),
                author: "2".to_string(),
                contest: "2".to_string(),
                statement: "2".to_string(),
                rating: 2,
            }, vec![Submission {
                id: 4,
                user_id: 4,
                problem_id: 2,
                source_code: "4".to_string(),
                score: 4,
                language: "4".to_string(),
            }, Submission {
                id: 5,
                user_id: 5,
                problem_id: 2,
                source_code: "5".to_string(),
                score: 100,
                language: "5".to_string(),
            }])
        ])
    }
}