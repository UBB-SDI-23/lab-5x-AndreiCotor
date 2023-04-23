use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationDTO {
    pub first_id: i32,
    pub last_id: i32,
    pub direction: i32,
    pub limit: i32
}

#[derive(Deserialize)]
pub struct ParticipationPaginationDTO {
    pub first_uid: i32,
    pub first_cid: i32,
    pub last_uid: i32,
    pub last_cid: i32,
    pub direction: i32,
    pub limit: i32
}