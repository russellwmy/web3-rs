use {
    super::{types::Commitment, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::signature::Signature,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<Encoding>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionRequest {
    signature: Signature,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetTransactionConfig>,
}

impl GetTransactionRequest {
    pub fn new(signature: &str) -> Self {
        let signature = Signature::from_str(signature).expect("invalid signature");

        Self {
            signature,
            config: None,
        }
    }
    pub fn new_with_config(signature: &str, config: GetTransactionConfig) -> Self {
        let signature = Signature::from_str(signature).expect("invalid signature");

        Self {
            signature,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetTransactionRequest {
    fn into(self) -> serde_json::Value {
        let signature = self.signature.to_string();

        match self.config {
            Some(config) => serde_json::json!([signature, config]),
            None => serde_json::json!([signature]),
        }
    }
}

impl Into<RpcRequest> for GetTransactionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getTransaction");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionResponse {
    block_time: u64,
    // TODO: implement type
    meta: serde_json::Value,
    slot: u64,
    // TODO: implement type
    transaction: serde_json::Value,
}

impl From<RpcResponse> for GetTransactionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
