use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockTimeRequest {
    pub slot: u64,
}

impl GetBlockTimeRequest {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }
}

impl Into<serde_json::Value> for GetBlockTimeRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.slot])
    }
}

impl Into<RpcRequest> for GetBlockTimeRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBlockTime");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockTimeResponse(i64);

impl From<RpcResponse> for GetBlockTimeResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
