use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksRequest {
    start_slot: u64,
    end_slot: Option<u64>,
    commitment: Option<Commitment>,
}

impl GetBlocksRequest {
    pub fn new(start_slot: u64, end_slot: Option<u64>, commitment: Option<Commitment>) -> Self {
        Self {
            start_slot,
            end_slot,
            commitment,
        }
    }
}

impl Into<serde_json::Value> for GetBlocksRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.start_slot, self.end_slot, self.commitment])
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
#[serde(rename_all = "camelCase")]
pub struct GetBlocksResponse(Vec<u64>);

impl From<RpcResponse> for GetBlocksResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
