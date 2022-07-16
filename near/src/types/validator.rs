use super::{AccountId, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub account_id: AccountId,
    pub is_slashed: Option<bool>,
    pub num_expected_blocks: Option<u64>,
    pub num_expected_chunks: Option<u64>,
    pub num_produced_blocks: Option<u64>,
    pub num_produced_chunks: Option<u64>,
    pub amount: Option<String>,
    pub public_key: Option<PublicKey>,
    pub stake: Option<String>,
    pub shards: Option<Vec<u64>>,
}
