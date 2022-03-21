use {
    super::{serde_utils::deserialize_public_key, types::Commitment},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVoteAccountsRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "votePubkey")]
    pub vote_public_key: Option<Pubkey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_unstaked_delinquents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent_slot_distance: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVoteAccountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetVoteAccountsRequestConfig>,
}

impl GetVoteAccountsRequest {
    pub fn new() -> Self {
        Self { config: None }
    }
    pub fn new_with_config(config: GetVoteAccountsRequestConfig) -> Self {
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
    pub vote_public_key: Pubkey,
    #[serde(deserialize_with = "deserialize_public_key", rename = "nodePubkey")]
    pub node_public_key: Pubkey,
    pub activated_stake: u64,
    pub epoch_vote_account: bool,
    pub commission: f64,
    pub last_vote: u64,
    pub root_slot: u64,
    // TODO: Convert this to a struct
    pub epoch_credits: Vec<(u64, u64, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVoteAccountsResponse {
    pub current: Vec<VoteAccountsValue>,
    pub delinquent: Vec<VoteAccountsValue>,
}

impl From<RpcResponse> for GetVoteAccountsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
