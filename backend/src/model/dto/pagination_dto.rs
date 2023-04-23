use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationDTO {
    pub first_id: i32,
    pub last_id: i32,
    pub direction: i32,
    pub limit: i32
}