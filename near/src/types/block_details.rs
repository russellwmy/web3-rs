use super::{BlockHash, BlockHeight, EpochId, GasPrice};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetails {
    pub height: BlockHeight,
    pub epoch_id: EpochId,
    pub next_epoch_id: EpochId,
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    // TODO: convert this to a type
    pub prev_state_root: String,
    // TODO: convert this to a type
    pub chunk_receipts_root: String,
    // TODO: convert this to a type
    pub chunk_headers_root: String,
    // TODO: convert this to a type
    pub chunk_tx_root: String,
    // TODO: convert this to a type
    pub outcome_root: String,
    pub chunks_included: u64,
    // TODO: convert this to a type
    pub challenges_root: String,
    pub timestamp: u64,
    pub timestamp_nanosec: String,
    pub random_value: String,
    // TODO: convert this to a type
    pub validator_proposals: Vec<serde_json::Value>,
    pub chunk_mask: Vec<bool>,
    pub gas_price: GasPrice,
    // TODO: convert this to a type
    pub rent_paid: String,
    // TODO: convert this to a type
    pub validator_reward: String,
    // TODO: convert this to a type
    pub total_supply: String,
    // TODO: convert this to a type
    pub challenges_result: Vec<serde_json::Value>,
    pub last_final_block: BlockHash,
    pub last_ds_final_block: BlockHash,
    pub next_bp_hash: BlockHash,
    pub block_merkle_root: String,
    // TODO: convert this to a type
    pub approvals: Vec<serde_json::Value>,
    // TODO: convert this to a signature type
    pub signature: serde_json::Value,
    pub latest_protocol_version: u64,
}
