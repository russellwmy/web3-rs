use {
    super::serde_utils::deserialize_public_key,
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVoteAccountsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "votePubkey")]
    vote_public_key: Option<Pubkey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_unstaked_delinquents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delinquent_slot_distance: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVoteAccountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetVoteAccountsConfig>,
}

impl GetVoteAccountsRequest {
    pub fn new() -> Self {
        Self { config: None }
    }
    pub fn new_with_config(config: GetVoteAccountsConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetVoteAccountsRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetVoteAccountsRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getVoteAccounts");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteAccountsValue {
    #[serde(deserialize_with = "deserialize_public_key", rename = "votePubkey")]
    vote_public_key: Pubkey,
    #[serde(deserialize_with = "deserialize_public_key", rename = "nodePubkey")]
    node_public_key: Pubkey,
    activated_stake: u64,
    epoch_vote_account: bool,
    commission: f64,
    last_vote: u64,
    root_slot: u64,
    epoch_credits: Vec<(u64, u64, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVoteAccountsResponse {
    current: Vec<VoteAccountsValue>,
    delinquent: Vec<VoteAccountsValue>,
}

impl From<RpcResponse> for GetVoteAccountsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
