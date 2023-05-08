use diesel::QueryResult;
use crate::model::contest::{NewContest, Contest, UpdContest};
use crate::model::dto::pagination_dto::PaginationDTO;
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn get_contests_paginated(db: &mut Mockable<DbConn>, pagination: PaginationDTO) -> Result<Vec<Contest>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_contests_paginated(inner, pagination),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_all_contests(db: &mut Mockable<DbConn>) -> Result<Vec<Contest>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_contests(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_contests_by_name(db: &mut Mockable<DbConn>, cname: Option<String>) -> Result<Vec<Contest>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_contests_by_name(inner, cname),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_contest_by_id(db: &mut Mockable<DbConn>, uid: i32) -> Result<Option<Contest>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_contest_by_id(inner, uid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn add_contest(db: &mut Mockable<DbConn>, con: NewContest) {
    match db {
        Mockable::Real(inner) => real::add_contest(inner, con),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn delete_contest(db: &mut Mockable<DbConn>, cid: i32) {
    match db {
        Mockable::Real(inner) => real::delete_contest(inner, cid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn update_contest(db: &mut Mockable<DbConn>, con: UpdContest) {
    match db {
        Mockable::Real(inner) => real::update_contest(inner, con),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_number_of_contests_by_uid(db: &mut Mockable<DbConn>, usid: i32) -> QueryResult<i64> {
    match db {
        Mockable::Real(inner) => real::get_number_of_contests_by_uid(inner, usid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn delete_all_contests(db: &mut Mockable<DbConn>) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::delete_all_contests(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::contest::{Contest, NewContest, UpdContest};
    use crate::model::dto::pagination_dto::PaginationDTO;
    use crate::repository::DbError;
    use crate::schema::contest::dsl::*;

    pub fn get_contests_paginated(db: &mut PgConnection, pagination: PaginationDTO) -> Result<Vec<Contest>, DbError> {
        let contest_list = if pagination.direction == 1 {
            contest.filter(id.gt(pagination.last_id))
                .order(id.asc())
                .limit(pagination.limit as i64)
                .load(db)?
        } else {
            contest.filter(id.lt(pagination.first_id))
                .order(id.desc())
                .limit(pagination.limit as i64)
                .load(db)?
        };

        Ok(contest_list)
    }

    pub fn get_all_contests(db: &mut PgConnection) -> Result<Vec<Contest>, DbError> {
        let contest_list = contest.load(db)?;
        Ok(contest_list)
    }

    pub fn get_contests_by_name(db: &mut PgConnection, cname: Option<String>) -> Result<Vec<Contest>, DbError> {
        let contest_list = match cname {
            Some(v) => contest.filter(name.like(format!("{}%", v)))
                .limit(10).load(db) ?,
            None => contest.limit(10).load(db)?
        };
        Ok(contest_list)
    }

    pub fn get_contest_by_id(db: &mut PgConnection, cid: i32) -> Result<Option<Contest>, DbError> {
        let con = contest.filter(id.eq(cid))
            .first::<Contest>(db)
            .optional()?;

        Ok(con)
    }

    pub fn get_number_of_contests_by_uid(db: &mut PgConnection, usid: i32) -> QueryResult<i64> {
        contest.filter(uid.eq(usid))
            .count()
            .get_result(db)
    }

    pub fn add_contest(db: &mut PgConnection, con: NewContest) {
        diesel::insert_into(contest).values(con).execute(db).unwrap();
    }

    pub fn delete_contest(db: &mut PgConnection, cid: i32) {
        diesel::delete(contest.filter(id.eq(cid))).execute(db).unwrap();
    }

    pub fn delete_all_contests(db: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(contest).execute(db)
    }

    pub fn update_contest(db: &mut PgConnection, con: UpdContest) {
        diesel::update(contest.filter(id.eq(con.id))).set(con).execute(db).unwrap();
    }
}