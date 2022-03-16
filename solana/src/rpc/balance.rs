use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceRequestConfig {
    pub commitment: Commitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    pub public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetBalanceRequestConfig>,
}

impl GetBalanceRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            config: None,
        }
    }
    pub fn new_with_config(public_key: Pubkey, config: GetBalanceRequestConfig) -> Self {
        Self {
            public_key,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetBalanceRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([self.public_key, config]),
            None => serde_json::json!([self.public_key.to_string()]),
        }
    }
}

impl Into<RpcRequest> for GetBalanceRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBalance");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub context: Context,
    pub value: u64,
}

impl From<RpcResponse> for GetBalanceResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
