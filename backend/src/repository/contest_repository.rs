use crate::model::contest::{NewContest, Contest};
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn get_all_contests(db: &mut Mockable<DbConn>) -> Result<Vec<Contest>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_contests(inner),
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

pub fn update_contest(db: &mut Mockable<DbConn>, con: Contest) {
    match db {
        Mockable::Real(inner) => real::update_contest(inner, con),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::contest::{Contest, NewContest};
    use crate::repository::DbError;
    use crate::schema::contest::dsl::*;

    pub fn get_all_contests(db: &mut PgConnection) -> Result<Vec<Contest>, DbError> {
        let contest_list = contest.load(db)?;
        Ok(contest_list)
    }

    pub fn get_contest_by_id(db: &mut PgConnection, uid: i32) -> Result<Option<Contest>, DbError> {
        let con = contest.filter(id.eq(uid))
            .first::<Contest>(db)
            .optional()?;

        Ok(con)
    }

    pub fn add_contest(db: &mut PgConnection, con: NewContest) {
        diesel::insert_into(contest).values(con).execute(db).unwrap();
    }

    pub fn delete_contest(db: &mut PgConnection, cid: i32) {
        diesel::delete(contest.filter(id.eq(cid))).execute(db).unwrap();
    }

    pub fn update_contest(db: &mut PgConnection, con: Contest) {
        diesel::update(contest.filter(id.eq(con.id))).set(con).execute(db).unwrap();
    }
}