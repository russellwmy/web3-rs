use super::{AccountId, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub account_id: AccountId,
    pub public_key: Option<PublicKey>,
    pub stake: String,
    pub validator_stake_struct_version: String,
}
