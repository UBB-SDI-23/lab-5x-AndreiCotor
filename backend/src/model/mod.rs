pub mod problem;
pub mod user;
pub mod submission;
pub mod dto;
pub mod contest;
pub mod participates;
pub mod user_credentials;
pub mod pagination_options;

pub trait ModelItem {
    fn get_id(&self) -> i32;
    fn set_id(&mut self, id: i32);
    fn update(&mut self, x: Self);
}