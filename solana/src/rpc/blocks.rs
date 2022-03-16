use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlocksRequestConfig {
    pub commitment: Commitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksRequest {
    pub start_slot: u64,
    pub end_slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetBlocksRequestConfig>,
}

impl GetBlocksRequest {
    pub fn new(start_slot: u64, end_slot: Option<u64>) -> Self {
        Self {
            start_slot,
            end_slot,
            config: None,
        }
    }
    pub fn new_with_config(
        start_slot: u64,
        end_slot: Option<u64>,
        config: GetBlocksRequestConfig,
    ) -> Self {
        Self {
            start_slot,
            end_slot,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetBlocksRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([self.start_slot, self.end_slot, config.commitment]),
            None => serde_json::json!([self.start_slot, self.end_slot]),
        }
    }
}

impl Into<RpcRequest> for GetBlocksRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBlocks");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlocksResponse(Vec<u64>);

impl From<RpcResponse> for GetBlocksResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
