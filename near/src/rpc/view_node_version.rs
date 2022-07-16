use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::NodeStatus,
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewNodeVersionRequest {}

impl ViewNodeVersionRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for ViewNodeVersionRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([])
    }
}

impl Into<RpcRequest> for ViewNodeVersionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("status");
        let params = self.into();

        request.params(params).clone()
    }
}

pub type ViewNodeVersionResponse = NodeStatus;

impl From<RpcResponse> for ViewNodeVersionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
