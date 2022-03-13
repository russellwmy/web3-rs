use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksWithLimitRequest {
    start_slot: u64,
    limit: u64,
    commitment: Option<Commitment>,
}

impl GetBlocksWithLimitRequest {
    pub fn new(start_slot: u64, limit: u64, commitment: Option<Commitment>) -> Self {
        Self {
            start_slot,
            limit,
            commitment,
        }
    }
}

impl Into<serde_json::Value> for GetBlocksWithLimitRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.start_slot, self.limit, self.commitment])
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
