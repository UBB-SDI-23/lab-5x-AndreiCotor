use diesel::prelude::*;
use crate::model::user_credentials::{NewUserCredentials, UserCredentials};
use crate::repository::{DbConn, DbError};
use crate::schema::usercredentials::username;
use crate::utils::mock::Mockable;

pub fn add_user_credentials(db: &mut Mockable<DbConn>, uc: NewUserCredentials) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::add_user_credentials(inner, uc),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn update_user_credentials(db: &mut Mockable<DbConn>, uc: UserCredentials) -> QueryResult<usize> {
    match db {
        Mockable::Real(inner) => real::update_user_credentials(inner, uc),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_user_credentials(db: &mut Mockable<DbConn>, usernm: String) -> Option<UserCredentials> {
    match db {
        Mockable::Real(inner) => real::get_user_credentials(inner, usernm),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_user_credentials_by_id(db: &mut Mockable<DbConn>, ucid: i32) -> Option<UserCredentials> {
    match db {
        Mockable::Real(inner) => real::get_user_credentials_by_id(inner, ucid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::user_credentials::{NewUserCredentials, UserCredentials};
    use crate::repository::DbError;
    use crate::schema::usercredentials::dsl::*;

    pub fn add_user_credentials(db: &mut PgConnection, uc: NewUserCredentials) -> QueryResult<usize> {
        diesel::insert_into(usercredentials)
            .values(uc)
            .execute(db)
    }

    pub fn update_user_credentials(db: &mut PgConnection, uc: UserCredentials) -> QueryResult<usize> {
        diesel::update(usercredentials.filter(id.eq(uc.id)))
            .set(uc)
            .execute(db)
    }

    pub fn get_user_credentials(db: &mut PgConnection, usernm: String) -> Option<UserCredentials> {
        usercredentials.filter(username.eq(usernm))
            .first::<UserCredentials>(db)
            .optional()
            .unwrap()
    }

    pub fn get_user_credentials_by_id(db: &mut PgConnection, ucid: i32) -> Option<UserCredentials> {
        usercredentials.filter(id.eq(ucid))
            .first::<UserCredentials>(db)
            .optional()
            .unwrap()
    }
}