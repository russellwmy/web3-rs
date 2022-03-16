use super::{AccountId, NodeVersion, SyncInfo, Validator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub version: NodeVersion,
    pub chain_id: String,
    pub protocol_version: u64,
    pub latest_protocol_version: u64,
    pub rpc_addr: String,
    pub validators: Vec<Validator>,
    pub sync_info: SyncInfo,
    pub validator_account_id: Option<AccountId>,
}
