use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateBlockhashConfig {
    blockhash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateBlockhashRequest {
    blockhash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ValidateBlockhashConfig>,
}

impl ValidateBlockhashRequest {
    pub fn new(blockhash: &str) -> Self {
        Self {
            blockhash: blockhash.to_owned(),
            config: None,
        }
    }
    pub fn new_with_config(blockhash: &str, config: ValidateBlockhashConfig) -> Self {
        Self {
            blockhash: blockhash.to_owned(),
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for ValidateBlockhashRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(_) => serde_json::json!([self.blockhash, self.config]),
            None => serde_json::json!([self.blockhash]),
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
    context: Context,
    value: bool,
}

impl From<RpcResponse> for ValidateBlockhashResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
