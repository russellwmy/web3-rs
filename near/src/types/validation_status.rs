use super::{Kickout, Proposal, Validator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStatus {
    pub current_fishermen: Vec<Validator>,
    pub current_proposals: Vec<Proposal>,
    pub current_validators: Vec<Validator>,
    pub epoch_height: u64,
    pub epoch_start_height: u64,
    pub next_fishermen: Vec<Validator>,
    pub next_validators: Vec<Validator>,
    pub prev_epoch_kickout: Vec<Kickout>,
}
