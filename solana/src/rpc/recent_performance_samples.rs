use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentPerformanceSamplesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
}

impl GetRecentPerformanceSamplesRequest {
    pub fn new() -> Self {
        Self { limit: None }
    }
    pub fn new_with_limit(limit: usize) -> Self {
        Self { limit: Some(limit) }
    }
}

impl Into<serde_json::Value> for GetRecentPerformanceSamplesRequest {
    fn into(self) -> serde_json::Value {
        match self.limit {
            Some(limit) => serde_json::json!([limit]),
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
    slot: u64,
    num_transactions: u64,
    num_slots: u64,
    sample_period_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentPerformanceSamplesResponse(Option<Vec<RpcPerfSample>>);

impl From<RpcResponse> for GetRecentPerformanceSamplesResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
