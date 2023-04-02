use diesel::prelude::*;
use crate::model::participates::Participates;
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn get_all_participation(db: &mut Mockable<DbConn>) -> Result<Vec<Participates>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_participation(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_participation_by_ids(db: &mut Mockable<DbConn>, usid: i32, coid: i32) -> Result<Option<Participates>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_participation_by_ids(inner, usid, coid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn add_participation(db: &mut Mockable<DbConn>, participation: Participates) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::add_participation(inner, participation),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn add_multiple_participations(db: &mut Mockable<DbConn>, participation_list: Vec<Participates>) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::add_multiple_participations(inner, participation_list),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn delete_participation(db: &mut Mockable<DbConn>, usid: i32, coid: i32) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::delete_participation(inner, usid, coid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn update_participation(db: &mut Mockable<DbConn>, part: Participates) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::update_participation(inner, part),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::participates::Participates;
    use crate::repository::DbError;
    use crate::schema::participates::dsl::*;

    pub fn get_all_participation(db: &mut PgConnection) -> Result<Vec<Participates>, DbError> {
        let participation_list = participates.load(db)?;
        Ok(participation_list)
    }

    pub fn get_participation_by_ids(db: &mut PgConnection, usid: i32, coid: i32) -> Result<Option<Participates>, DbError> {
        let participation = participates.filter(uid.eq(usid))
            .filter(cid.eq(coid))
            .first::<Participates>(db)
            .optional()?;

        Ok(participation)
    }

    pub fn add_participation(db: &mut PgConnection, participation: Participates) -> QueryResult<usize> {
        diesel::insert_into(participates).values(participation).execute(db)
    }

    pub fn add_multiple_participations(db: &mut PgConnection, participation_list: Vec<Participates>) -> QueryResult<usize> {
        diesel::insert_into(participates).values(participation_list).execute(db)
    }

    pub fn delete_participation(db: &mut PgConnection, usid: i32, coid: i32) -> QueryResult<usize> {
        diesel::delete(participates.filter(uid.eq(usid)).filter(cid.eq(coid))).execute(db)
    }

    pub fn update_participation(db: &mut PgConnection, part: Participates) -> QueryResult<usize> {
        diesel::update(participates.filter(uid.eq(part.uid)).filter(cid.eq(part.cid))).set(part).execute(db)
    }
}