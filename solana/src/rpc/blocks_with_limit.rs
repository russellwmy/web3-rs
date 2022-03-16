use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlocksWithLimitRequestConfig {
    pub commitment: Commitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksWithLimitRequest {
    pub start_slot: u64,
    pub limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetBlocksWithLimitRequestConfig>,
}

impl GetBlocksWithLimitRequest {
    pub fn new(start_slot: u64, limit: u64) -> Self {
        Self {
            start_slot,
            limit,
            config: None,
        }
    }
    pub fn new_with_config(
        start_slot: u64,
        limit: u64,
        config: GetBlocksWithLimitRequestConfig,
    ) -> Self {
        Self {
            start_slot,
            limit,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetBlocksWithLimitRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([self.start_slot, self.limit, config.commitment]),
            None => serde_json::json!([self.start_slot, self.limit]),
        }
    }
}

impl Into<RpcRequest> for GetBlocksWithLimitRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBlocksWithLimit");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksWithLimitResponse(Vec<u64>);

impl From<RpcResponse> for GetBlocksWithLimitResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
