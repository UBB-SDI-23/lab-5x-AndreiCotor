use serde::{Serialize};
use crate::model::contest::Contest;
use crate::model::participates::Participates;
use crate::model::user::User;

#[derive(Serialize, Clone, Debug)]
pub struct ParticipatesDTO {
    #[serde(flatten)]
    pub participates: Participates,
    pub user: User,
    pub contest: Contest,
}