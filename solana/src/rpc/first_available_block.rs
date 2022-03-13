use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFirstAvailableBlockRequest {}

impl GetFirstAvailableBlockRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetFirstAvailableBlockRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetFirstAvailableBlockRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getFirstAvailableBlock");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFirstAvailableBlockResponse(u64);

impl From<RpcResponse> for GetFirstAvailableBlockResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
