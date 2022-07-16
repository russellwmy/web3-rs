use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::NetworkInfo,
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewNetworkInfoRequest {}

impl ViewNetworkInfoRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for ViewNetworkInfoRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([])
    }
}

impl Into<RpcRequest> for ViewNetworkInfoRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("network_info");
        let params = self.into();

        request.params(params).clone()
    }
}

pub type ViewNetworkInfoResponse = NetworkInfo;

impl From<RpcResponse> for ViewNetworkInfoResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
