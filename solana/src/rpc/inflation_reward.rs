use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationRewardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epoch: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationRewardRequest {
    addresses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetInflationRewardConfig>,
}

impl GetInflationRewardRequest {
    pub fn new(addresses: Vec<String>) -> Self {
        Self {
            addresses,
            config: None,
        }
    }
    pub fn new_with_config(addresses: Vec<String>, config: GetInflationRewardConfig) -> Self {
        Self {
            addresses,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetInflationRewardRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([self.addresses, config]),
            None => serde_json::json!([self.addresses]),
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
    epoch: f64,
    effective_slot: f64,
    amount: f64,
    post_balance: f64,
    commission: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationRewardResponse(Vec<Option<InflationRewardValue>>);

impl From<RpcResponse> for GetInflationRewardResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
