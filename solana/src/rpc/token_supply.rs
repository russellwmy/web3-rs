use {
    super::{serde_utils::deserialize_public_key, types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenSupplyRequestConfig {
    pub commitment: Commitment,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenSupplyRequest {
    pub public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetTokenSupplyRequestConfig>,
}

impl GetTokenSupplyRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            config: None,
        }
    }
    pub fn new_with_config(public_key: Pubkey, config: GetTokenSupplyRequestConfig) -> Self {
        Self {
            public_key,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetTokenSupplyRequest {
    fn into(self) -> serde_json::Value {
        let public_key = self.public_key.to_string();

        match self.config {
            Some(config) => serde_json::json!([public_key, config]),
            None => serde_json::json!([public_key]),
        }
    }
}

impl Into<RpcRequest> for GetTokenSupplyRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getTokenSupply");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSupplyValue {
    #[serde(deserialize_with = "deserialize_public_key")]
    pub address: Pubkey,
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<u64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenSupplyResponse {
    pub context: Context,
    pub value: Vec<TokenSupplyValue>,
}

impl From<RpcResponse> for GetTokenSupplyResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
