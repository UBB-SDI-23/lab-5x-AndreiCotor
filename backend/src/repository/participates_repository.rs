use diesel::prelude::*;
use crate::model::dto::pagination_dto::ParticipationPaginationDTO;
use crate::model::participates::Participates;
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn get_participation_paginated(db: &mut Mockable<DbConn>, pagination: ParticipationPaginationDTO) -> Result<Vec<Participates>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_participation_paginated(inner, pagination),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_participation_by_ids(db: &mut Mockable<DbConn>, usid: i32, coid: i32) -> Result<Option<Participates>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_participation_by_ids(inner, usid, coid),
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

pub fn get_participation_by_cid(db: &mut Mockable<DbConn>, coid: i32) -> Result<Vec<Participates>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_participation_by_cid(inner, coid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_number_of_participation_by_uid(db: &mut Mockable<DbConn>, usid: i32) -> QueryResult<i64> {
    match db {
        Mockable::Real(inner) => real::get_number_of_participation_by_uid(inner, usid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn delete_all_participations(db: &mut Mockable<DbConn>) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::delete_all_participations(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::dto::pagination_dto::ParticipationPaginationDTO;
    use crate::model::participates::Participates;
    use crate::repository::DbError;
    use crate::schema::participates::dsl::*;

    pub fn get_participation_paginated(db: &mut PgConnection, pagination: ParticipationPaginationDTO) -> Result<Vec<Participates>, DbError> {
        let participates_list = if pagination.direction == 1 {
            participates.filter(uid.gt(pagination.last_uid).or(uid.eq(pagination.last_uid).and(cid.gt(pagination.last_cid))))
                .order((uid.asc(), cid.asc()))
                .limit(pagination.limit as i64)
                .load(db)?
        } else {
            participates.filter(uid.lt(pagination.first_uid).or(uid.eq(pagination.first_uid).and(cid.lt(pagination.first_cid))))
                .order((uid.desc(), cid.desc()))
                .limit(pagination.limit as i64)
                .load(db)?
        };

        Ok(participates_list)
    }

    pub fn get_participation_by_ids(db: &mut PgConnection, usid: i32, coid: i32) -> Result<Option<Participates>, DbError> {
        let participation = participates.filter(uid.eq(usid))
            .filter(cid.eq(coid))
            .first::<Participates>(db)
            .optional()?;

        Ok(participation)
    }

    pub fn get_participation_by_cid(db: &mut PgConnection, coid: i32) -> Result<Vec<Participates>, DbError> {
        let participation_list = participates.filter(cid.eq(coid)).load(db)?;
        Ok(participation_list)
    }

    pub fn get_number_of_participation_by_uid(db: &mut PgConnection, usid: i32) -> QueryResult<i64> {
        participates.filter(cid.eq(usid))
            .count()
            .get_result(db)
    }

    pub fn add_multiple_participations(db: &mut PgConnection, participation_list: Vec<Participates>) -> QueryResult<usize> {
        diesel::insert_into(participates).values(participation_list).execute(db)
    }

    pub fn delete_participation(db: &mut PgConnection, usid: i32, coid: i32) -> QueryResult<usize> {
        diesel::delete(participates.filter(uid.eq(usid)).filter(cid.eq(coid))).execute(db)
    }

    pub fn delete_all_participations(db: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(participates).execute(db)
    }

    pub fn update_participation(db: &mut PgConnection, part: Participates) -> QueryResult<usize> {
        diesel::update(participates.filter(uid.eq(part.uid)).filter(cid.eq(part.cid))).set(part).execute(db)
    }
}