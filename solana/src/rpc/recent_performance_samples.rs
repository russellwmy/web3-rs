use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecentPerformanceSamplesRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentPerformanceSamplesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetRecentPerformanceSamplesRequestConfig>,
}

impl GetRecentPerformanceSamplesRequest {
    pub fn new() -> Self {
        Self { config: None }
    }
    pub fn new_with_config(config: GetRecentPerformanceSamplesRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetRecentPerformanceSamplesRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config.limit]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetRecentPerformanceSamplesRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getRecentPerformanceSamples");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcPerfSample {
    pub slot: u64,
    pub num_transactions: u64,
    pub num_slots: u64,
    pub sample_period_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentPerformanceSamplesResponse(Option<Vec<RpcPerfSample>>);

impl From<RpcResponse> for GetRecentPerformanceSamplesResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
