use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakeActivationRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakeActivationRequest {
    public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetStakeActivationRequestConfig>,
}

impl GetStakeActivationRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            config: None,
        }
    }

    pub fn new_with_config(public_key: Pubkey, config: GetStakeActivationRequestConfig) -> Self {
        Self {
            public_key,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetStakeActivationRequest {
    fn into(self) -> serde_json::Value {
        let public_key = self.public_key.to_string();

        match self.config {
            Some(config) => serde_json::json!([public_key, config]),
            None => serde_json::json!([public_key]),
        }
    }
}

impl Into<RpcRequest> for GetStakeActivationRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getStakeActivation");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakeActivationResponse {
    // TODO: Convert to an enum
    pub state: String,
    pub active: u64,
    pub inactive: u64,
}

impl From<RpcResponse> for GetStakeActivationResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
