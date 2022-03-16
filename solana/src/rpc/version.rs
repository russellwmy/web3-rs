use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionRequest {}

impl GetVersionRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetVersionRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetVersionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getVersion");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionResponse {
    #[serde(rename = "solana-core")]
    pub solana_core: String,
    #[serde(rename = "feature-core")]
    pub feature_set: Option<String>,
}

impl From<RpcResponse> for GetVersionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
