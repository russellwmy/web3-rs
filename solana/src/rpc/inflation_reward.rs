use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationRewardRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationRewardRequest {
    pub addresses: Vec<Pubkey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetInflationRewardRequestConfig>,
}

impl GetInflationRewardRequest {
    pub fn new(addresses: Vec<Pubkey>) -> Self {
        Self {
            addresses,
            config: None,
        }
    }
    pub fn new_with_config(
        addresses: Vec<Pubkey>,
        config: GetInflationRewardRequestConfig,
    ) -> Self {
        Self {
            addresses,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetInflationRewardRequest {
    fn into(self) -> serde_json::Value {
        let addresses = self
            .addresses
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>();

        match self.config {
            Some(config) => serde_json::json!([addresses, config]),
            None => serde_json::json!([addresses]),
        }
    }
}

impl Into<RpcRequest> for GetInflationRewardRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getInflationReward");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InflationRewardValue {
    pub epoch: f64,
    pub effective_slot: f64,
    pub amount: f64,
    pub post_balance: f64,
    pub commission: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationRewardResponse(Vec<Option<InflationRewardValue>>);

impl From<RpcResponse> for GetInflationRewardResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
