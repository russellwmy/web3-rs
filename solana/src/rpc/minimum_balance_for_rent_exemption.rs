use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinimumBalanceForRentExemptionRequestConfig {
    pub commitment: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinimumBalanceForRentExemptionRequest {
    pub data_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetMinimumBalanceForRentExemptionRequestConfig>,
}

impl GetMinimumBalanceForRentExemptionRequest {
    pub fn new(data_length: usize) -> Self {
        Self {
            data_length,
            config: None,
        }
    }
    pub fn new_with_config(
        data_length: usize,
        config: GetMinimumBalanceForRentExemptionRequestConfig,
    ) -> Self {
        Self {
            data_length,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetMinimumBalanceForRentExemptionRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => {
                serde_json::json!([self.data_length, config])
            }
            None => serde_json::json!([self.data_length]),
        }
    }
}

impl Into<RpcRequest> for GetMinimumBalanceForRentExemptionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getMinimumBalanceForRentExemption");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinimumBalanceForRentExemptionResponse(u64);

impl From<RpcResponse> for GetMinimumBalanceForRentExemptionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
