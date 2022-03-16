use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::hash::Hash,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateBlockhashRequestConfig {
    pub blockhash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateBlockhashRequest {
    blockhash: Hash,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ValidateBlockhashRequestConfig>,
}

impl ValidateBlockhashRequest {
    pub fn new(blockhash: Hash) -> Self {
        Self {
            blockhash,
            config: None,
        }
    }
    pub fn new_with_config(blockhash: Hash, config: ValidateBlockhashRequestConfig) -> Self {
        Self {
            blockhash,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for ValidateBlockhashRequest {
    fn into(self) -> serde_json::Value {
        let blockhash = self.blockhash.to_string();

        match self.config {
            Some(_) => serde_json::json!([blockhash, self.config]),
            None => serde_json::json!([blockhash]),
        }
    }
}

impl Into<RpcRequest> for ValidateBlockhashRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("isBlockhashValid");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateBlockhashResponse {
    pub context: Context,
    pub value: bool,
}

impl From<RpcResponse> for ValidateBlockhashResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
