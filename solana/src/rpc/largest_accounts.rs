use {
    super::{serde_utils::deserialize_public_key, types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLargestAccountsRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLargestAccountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetLargestAccountsRequestConfig>,
}

impl GetLargestAccountsRequest {
    pub fn new() -> Self {
        Self { config: None }
    }
    pub fn new_with_config(config: GetLargestAccountsRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetLargestAccountsRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetLargestAccountsRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getLargestAccounts");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargestAccountsValue {
    pub lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    pub address: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLargestAccountsResponse {
    pub context: Context,
    pub value: Vec<LargestAccountsValue>,
}

impl From<RpcResponse> for GetLargestAccountsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
