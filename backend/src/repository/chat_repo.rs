use diesel::QueryResult;
use crate::model::chat::Chat;
use crate::repository::DbConn;
use crate::utils::mock::Mockable;

pub fn add_chat(db: &mut Mockable<DbConn>, cht: Chat) {
    match db {
        Mockable::Real(inner) => real::add_chat(inner, cht),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::chat::Chat;
    use crate::schema::chat::dsl::chat;

    pub fn add_chat(db: &mut PgConnection, cht: Chat) {
        diesel::insert_into(chat).values(cht).execute(db).unwrap();
    }
}