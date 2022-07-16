use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::GenesisConfig,
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewGenesisConfigRequest {}

impl ViewGenesisConfigRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for ViewGenesisConfigRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for ViewGenesisConfigRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("EXPERIMENTAL_genesis_config");
        let params = self.into();

        request.params(params).clone()
    }
}

pub type ViewGenesisConfigResponse = GenesisConfig;

impl From<RpcResponse> for ViewGenesisConfigResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
