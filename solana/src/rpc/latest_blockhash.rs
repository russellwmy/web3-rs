use {
    super::{serde_utils::deserialize_hash, types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::hash::Hash,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestBlockhashRequestConfig {
    pub commitment: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestBlockhashRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetLatestBlockhashRequestConfig>,
}

impl GetLatestBlockhashRequest {
    pub fn new() -> Self {
        Self { config: None }
    }
    pub fn new_with_config(config: GetLatestBlockhashRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetLatestBlockhashRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetLatestBlockhashRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getLatestBlockhash");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestBlockhashValue {
    #[serde(deserialize_with = "deserialize_hash")]
    pub blockhash: Hash,
    pub last_valid_block_height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestBlockhashResponse {
    pub context: Context,
    pub value: LatestBlockhashValue,
}

impl From<RpcResponse> for GetLatestBlockhashResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
