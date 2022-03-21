use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotLeaderRequestConfig {
    pub commitment: Commitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotLeaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetSlotLeaderRequestConfig>,
}

impl GetSlotLeaderRequest {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn new_with_config(config: GetSlotLeaderRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetSlotLeaderRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config.commitment]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetSlotLeaderRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getSlotLeader");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotLeaderResponse(Pubkey);

impl Into<Pubkey> for GetSlotLeaderResponse {
    fn into(self) -> Pubkey {
        self.0
    }
}

impl From<RpcResponse> for GetSlotLeaderResponse {
    fn from(response: RpcResponse) -> Self {
        let public_key = response.result.as_str().expect("public key is a string");
        GetSlotLeaderResponse(Pubkey::from_str(public_key).expect("public key is valid"))
    }
}
