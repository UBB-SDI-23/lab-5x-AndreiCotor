use serde::{Serialize};
use crate::model::contest::Contest;

#[derive(Serialize, Clone, Debug)]
pub struct ContestDTO {
    #[serde(flatten)]
    pub contest: Contest,
    pub cnt: i32
}