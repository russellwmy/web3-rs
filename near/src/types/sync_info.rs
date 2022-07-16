use super::{BlockHash, BlockHeight};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncInfo {
    pub earliest_block_hash: BlockHash,
    pub earliest_block_height: BlockHeight,
    pub earliest_block_time: String,
    pub latest_block_hash: BlockHash,
    pub latest_block_height: BlockHeight,
    pub latest_state_root: BlockHash,
    pub latest_block_time: String,
    pub syncing: bool,
}
