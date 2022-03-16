#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkDetails {
    // TODO: convert this to a type
    pub chunk_hash: String,
    // TODO: convert this to a type
    pub prev_block_hash: String,
    // TODO: convert this to a type
    pub outcome_root: String,
    // TODO: convert this to a type
    pub prev_state_root: String,
    // TODO: convert this to a type
    pub encoded_merkle_root: String,
    pub encoded_length: u64,
    pub height_created: u64,
    pub height_included: u64,
    pub shard_id: u64,
    pub gas_used: u64,
    pub gas_limit: u64,
    // TODO: convert this to a type
    pub rent_paid: String,
    // TODO: convert this to a type
    pub validator_reward: String,
    // TODO: convert this to a type
    pub balance_burnt: String,
    // TODO: convert this to a type
    pub outgoing_receipts_root: String,
    // TODO: convert this to a type
    pub tx_root: String,
    // TODO: convert this to a type
    pub validator_proposals: Vec<serde_json::Value>,
    // TODO: convert this to a signature type
    pub signature: serde_json::Value,
}
