use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEpochInfoRequestConfig {
    pub commitment: Commitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEpochInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetEpochInfoRequestConfig>,
}

impl GetEpochInfoRequest {
    pub fn new() -> Self {
        Self { config: None }
    }
    pub fn new_with_config(config: GetEpochInfoRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetEpochInfoRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config]),
            None => serde_json::json!([]),
        }
    }
}

impl Into<RpcRequest> for GetEpochInfoRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getEpochInfo");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEpochInfoResponse {
    pub absolute_slot: u64,
    pub block_height: u64,
    pub epoch: u64,
    pub slot_index: u64,
    pub slots_in_epoch: u64,
    pub transaction_count: Option<u64>,
}

impl From<RpcResponse> for GetEpochInfoResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
