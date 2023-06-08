use diesel::QueryResult;
use crate::model::dto::pagination_dto::{PaginationDTO, StatisticPagination};
use crate::model::problem::{NewProblem, Problem, UpdProblem};
use crate::model::submission::Submission;
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn get_problems_paginated(db: &mut Mockable<DbConn>, pagination: PaginationDTO) -> Result<Vec<Problem>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problems_paginated(inner, pagination),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn number_of_problems(db: &mut Mockable<DbConn>) -> Result<i32, DbError> {
    match db {
        Mockable::Real(inner) => real::number_of_problems(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}


pub fn get_problem_by_name(db: &mut Mockable<DbConn>, pname: Option<String>) -> Result<Vec<Problem>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problem_by_name(inner, pname),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_problem_by_id(db: &mut Mockable<DbConn>, uid: i32) -> Result<Option<Problem>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problem_by_id(inner, uid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_problems_rating_larger(db: &mut Mockable<DbConn>, problem_rating: i32, pagination: PaginationDTO) -> Result<Vec<Problem>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problems_rating_larger(inner, problem_rating, pagination),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_problems_by_submissions(db: &mut Mockable<DbConn>, pagination: StatisticPagination) -> Result<Vec<(Problem, i32)>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_problems_by_submissions(inner, pagination),
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

pub fn update_problem(db: &mut Mockable<DbConn>, problem: UpdProblem) {
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

pub fn get_number_of_problems_by_uid(db: &mut Mockable<DbConn>, usid: i32) -> QueryResult<i64> {
    match db {
        Mockable::Real(inner) => real::get_number_of_problems_by_uid(inner, usid),
        Mockable::Mock => panic!("Not implemented")
    }
}

pub fn delete_all_problems(db: &mut Mockable<DbConn>) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::delete_all_problems(inner),
        Mockable::Mock => panic!("Not implemented")
    }
}

mod real {
    use diesel::prelude::*;
    use diesel::sql_query;
    use crate::model::dto::pagination_dto::{PaginationDTO, StatisticPagination};
    use crate::model::problem::{NewProblem, Problem, UpdProblem};
    use crate::model::submission::Submission;
    use crate::repository::DbError;
    use crate::schema::problems::dsl::*;
    use diesel::sql_types::Integer;

    #[derive(QueryableByName, Debug)]
    struct Auxiliary {
        #[diesel(sql_type = Integer)]
        pub pid: i32,
        #[diesel(sql_type = Integer)]
        pub cnt: i32
    }

    pub fn get_problems_paginated(db: &mut PgConnection, pagination: PaginationDTO) -> Result<Vec<Problem>, DbError> {
        let problem_list = if pagination.direction == 1 {
            problems.filter(id.gt(pagination.last_id))
                .order(id.asc())
                .limit(pagination.limit as i64)
                .load(db)?
        } else {
            problems.filter(id.lt(pagination.first_id))
                .order(id.desc())
                .limit(pagination.limit as i64)
                .load(db)?
        };

        Ok(problem_list)
    }

    pub fn number_of_problems(db: &mut PgConnection) -> Result<i32, DbError> {
        let cnt: i64 = problems.count().get_result(db).unwrap();
        Ok(cnt as i32)
    }

    pub fn get_problem_by_name(db: &mut PgConnection, pname: Option<String>) -> Result<Vec<Problem>, DbError> {
        let problem_list = match pname {
            Some(v) => problems.filter(name.like(format!("{}%", v)))
                .limit(10).load(db) ?,
            None => problems.limit(10).load(db)?
        };
        Ok(problem_list)
    }

    pub fn get_number_of_problems_by_uid(db: &mut PgConnection, usid: i32) -> QueryResult<i64> {
        problems.filter(uid.eq(usid))
            .count()
            .get_result(db)
    }

    pub fn get_problem_by_id(db: &mut PgConnection, pid: i32) -> Result<Option<Problem>, DbError> {
        let problem = problems.filter(id.eq(pid))
            .first::<Problem>(db)
            .optional()?;

        Ok(problem)
    }

    pub fn get_problems_rating_larger(db: &mut PgConnection, problem_rating: i32, pagination: PaginationDTO) -> Result<Vec<Problem>, DbError> {
        let problem_list = if pagination.direction == 1 {
            problems.filter(rating.gt(problem_rating).and(id.gt(pagination.last_id)))
                .order(id.asc())
                .limit(pagination.limit as i64)
                .load(db)?
        } else {
            problems.filter(rating.gt(problem_rating).and(id.lt(pagination.first_id)))
                .order(id.desc())
                .limit(pagination.limit as i64)
                .load(db)?
        };

        Ok(problem_list)
    }

    pub fn add_problem(db: &mut PgConnection, problem: NewProblem) {
        diesel::insert_into(problems).values(problem).execute(db).unwrap();
    }

    pub fn delete_problem(db: &mut PgConnection, pid: i32) {
        diesel::delete(problems.filter(id.eq(pid))).execute(db).unwrap();
    }

    pub fn delete_all_problems(db: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(problems).execute(db)
    }

    pub fn update_problem(db: &mut PgConnection, problem: UpdProblem) {
        diesel::update(problems.filter(id.eq(problem.id))).set(problem).execute(db).unwrap();
    }

    pub fn get_problems_by_submissions(db: &mut PgConnection, pagination: StatisticPagination) -> Result<Vec<(Problem, i32)>, DbError> {
        let auxiliary_list =  if pagination.direction == 1 {
            sql_query(format!("SELECT * FROM PROBLEMSSUBMISSIONS WHERE CNT > {} OR (CNT = {} AND PID > {}) ORDER BY CNT, PID limit {}", pagination.last_stat, pagination.last_stat, pagination.last_id, pagination.limit))
                .get_results::<Auxiliary>(db)?
        }
        else {
            sql_query(format!("SELECT * FROM PROBLEMSSUBMISSIONS WHERE CNT < {} OR (CNT = {} AND PID < {}) ORDER BY CNT DESC, PID DESC limit {}", pagination.first_stat, pagination.first_stat, pagination.first_id, pagination.limit))
                .get_results::<Auxiliary>(db)?
        };

        let mut problem_list = vec![];
        for el in auxiliary_list {
            problem_list.push((get_problem_by_id(db,el.pid).unwrap().unwrap(),el.cnt));
        }

        Ok(problem_list)
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
                uid: 1,
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
                uid: 2,
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