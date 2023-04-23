use diesel::prelude::*;
use crate::model::dto::pagination_dto::PaginationDTO;
use crate::model::submission::{NewSubmission, Submission};
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn add_submission(db: &mut Mockable<DbConn>, submission: NewSubmission) {
    match db {
        Mockable::Real(inner) => real::add_submission(inner, submission),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_all_submissions(db: &mut Mockable<DbConn>) -> Result<Vec<Submission>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_submissions(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_submissions_paginated(db: &mut Mockable<DbConn>, pagination: PaginationDTO) -> Result<Vec<Submission>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_submissions_paginated(inner, pagination),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_submission(db: &mut Mockable<DbConn>, sid: i32) -> Result<Option<Submission>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_submission(inner, sid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_all_submissions_by_problem_id(db: &mut Mockable<DbConn>, pid: i32) -> Result<Vec<Submission>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_submissions_by_problem_id(inner, pid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_all_submissions_by_user_id(db: &mut Mockable<DbConn>, uid: i32) -> Result<Vec<Submission>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_submissions_by_user_id(inner, uid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn delete_submission(db: &mut Mockable<DbConn>, sid: i32) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::delete_submission(inner, sid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn update_submission(db: &mut Mockable<DbConn>, sub: Submission) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::update_submission(inner, sub),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::dto::pagination_dto::PaginationDTO;
    use crate::model::submission::{NewSubmission, Submission};
    use crate::repository::DbError;
    use crate::schema::submissions::dsl::*;

    pub fn add_submission(db: &mut PgConnection, submission: NewSubmission) {
        diesel::insert_into(submissions).values(submission).execute(db).unwrap();
    }

    pub fn get_submissions_paginated(db: &mut PgConnection, pagination: PaginationDTO) -> Result<Vec<Submission>, DbError> {
        let submission_list = if pagination.direction == 1 {
            submissions.filter(id.gt(pagination.last_id))
                .order(id.asc())
                .limit(pagination.limit as i64)
                .load(db)?
        } else {
            submissions.filter(id.lt(pagination.first_id))
                .order(id.desc())
                .limit(pagination.limit as i64)
                .load(db)?
        };

        Ok(submission_list)
    }

    pub fn get_all_submissions(db: &mut PgConnection) -> Result<Vec<Submission>, DbError> {
        let submission_list = submissions.load(db)?;
        Ok(submission_list)
    }

    pub fn get_submission(db: &mut PgConnection, sid: i32) -> Result<Option<Submission>, DbError> {
        let submission = submissions.filter(id.eq(sid)).first::<Submission>(db).optional().unwrap();
        Ok(submission)
    }

    pub fn get_all_submissions_by_problem_id(db: &mut PgConnection, pid: i32) -> Result<Vec<Submission>, DbError> {
        let submission_list = submissions.filter(problem_id.eq(pid)).load(db).unwrap();
        Ok(submission_list)
    }

    pub fn get_all_submissions_by_user_id(db: &mut PgConnection, uid: i32) -> Result<Vec<Submission>, DbError> {
        let submission_list = submissions.filter(user_id.eq(uid)).load(db).unwrap();
        Ok(submission_list)
    }

    pub fn delete_submission(db: &mut PgConnection, sid: i32) -> QueryResult<usize> {
        diesel::delete(submissions.filter(id.eq(sid))).execute(db)
    }

    pub fn update_submission(db: &mut PgConnection, sub: Submission) -> QueryResult<usize> {
        diesel::update(submissions.filter(id.eq(sub.id))).set(sub).execute(db)
    }
}