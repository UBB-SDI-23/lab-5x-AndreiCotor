use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::participates;
use crate::model::user::User;
use crate::model::contest::Contest;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable, Identifiable, Associations)]
#[diesel(table_name = participates)]
#[diesel(belongs_to(User, foreign_key = uid))]
#[diesel(belongs_to(Contest, foreign_key = cid))]
#[diesel(primary_key(uid, cid))]
pub struct Participates {
    pub uid: i32,
    pub cid: i32,
    pub score: i32,
    pub official: bool
}

impl Participates {
    pub fn is_valid(&self) -> bool {
        0 <= self.score
    }
}