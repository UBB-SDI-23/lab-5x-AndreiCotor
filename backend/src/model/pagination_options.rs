use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::pagoptions;

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, AsChangeset, Insertable, Identifiable)]
#[diesel(table_name = pagoptions)]
pub struct PagOption {
    pub id: i32,
    pub pages: i32
}

impl PagOption {
    pub fn valid(&self) -> bool {
        self.pages >= 1 && self.pages <= 50
    }
}