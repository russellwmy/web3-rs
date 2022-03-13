use {
    super::serde_utils::deserialize_public_key,
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenSupplyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenSupplyRequest {
    public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetTokenSupplyConfig>,
}

impl GetTokenSupplyRequest {
    pub fn new(public_key: &str) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");

        Self {
            public_key,
            config: None,
        }
    }
    pub fn new_with_config(public_key: &str, config: GetTokenSupplyConfig) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");

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
pub struct TokenSupplyValue {
    #[serde(deserialize_with = "deserialize_public_key")]
    address: Pubkey,
    amount: String,
    decimals: u8,
    ui_amount: Option<u64>,
    ui_amount_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenSupplyResponse {
    context: Context,
    value: Vec<TokenSupplyValue>,
}

impl From<RpcResponse> for GetTokenSupplyResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
