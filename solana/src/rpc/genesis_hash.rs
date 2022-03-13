use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGenesisHashRequest {}

impl GetGenesisHashRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetGenesisHashRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetGenesisHashRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getGenesisHash");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGenesisHashResponse(String);

impl From<RpcResponse> for GetGenesisHashResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
