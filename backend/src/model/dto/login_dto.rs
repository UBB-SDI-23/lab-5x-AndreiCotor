use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginDTO {
    pub id: i32,
    pub token: String,
    pub username: String,
    pub role: String
}