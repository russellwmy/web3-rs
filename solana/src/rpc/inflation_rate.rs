use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationRateRequest {}

impl GetInflationRateRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetInflationRateRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetInflationRateRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getInflationRate");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInflationRateResponse {
    total: f64,
    validator: f64,
    foundation: f64,
    epoch: f64,
}

impl From<RpcResponse> for GetInflationRateResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
