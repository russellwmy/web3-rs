use super::{BlockHash, BlockHeight, Permission};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessKey {
    nonce: u64,
    permission: Permission,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_height: Option<BlockHeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_hash: Option<BlockHash>,
}
