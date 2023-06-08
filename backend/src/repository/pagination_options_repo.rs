use diesel::QueryResult;
use crate::model::pagination_options::PagOption;
use crate::repository::{DbConn, DbError};
use crate::utils::mock::Mockable;

pub fn get_number_of_pages(db: &mut Mockable<DbConn>) -> Result<Option<PagOption>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_number_of_pages(inner),
        Mockable::Mock => panic!("Not implemented!")
    }
}

pub fn set_number_of_pages(db: &mut Mockable<DbConn>, pg: i32) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::set_number_of_pages(inner, pg),
        Mockable::Mock => panic!("Not implemented!")
    }
}

mod real {
    use diesel::PgConnection;
    use crate::model::pagination_options::PagOption;
    use crate::repository::DbError;
    use diesel::prelude::*;
    use crate::schema::pagoptions::dsl::*;

    pub fn get_number_of_pages(db: &mut PgConnection) -> Result<Option<PagOption>, DbError> {
        let opt = pagoptions.filter(id.eq(1))
            .first::<PagOption>(db)
            .optional()
            .unwrap();

        Ok(opt)
    }

    pub fn set_number_of_pages(db: &mut PgConnection, pg: i32) -> QueryResult<usize> {
        let pgopt = PagOption{id: 1, pages: pg};

        diesel::update(pagoptions.filter(id.eq(1)))
            .set(pgopt)
            .execute(db)
    }
}