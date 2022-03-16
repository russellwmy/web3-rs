use {
    super::{types::Commitment, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::signature::Signature,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTransactionRequest {
    pub signature: Signature,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetTransactionRequestConfig>,
}

impl GetTransactionRequest {
    pub fn new(signature: Signature) -> Self {
        Self {
            signature,
            config: None,
        }
    }
    pub fn new_with_config(signature: Signature, config: GetTransactionRequestConfig) -> Self {
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
    pub block_time: u64,
    // TODO: Convert this to a struct
    pub meta: serde_json::Value,
    pub slot: u64,
    // TODO: Convert this to a struct
    pub transaction: serde_json::Value,
}

impl From<RpcResponse> for GetTransactionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
