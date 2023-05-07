use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: i32,
    pub role: String
}