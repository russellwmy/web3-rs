use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHealthRequest {}

impl GetHealthRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetHealthRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetHealthRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getHealth");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorValue {
    code: i32,
    message: String,
    data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHealthResponse(String);

impl From<RpcResponse> for GetHealthResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
