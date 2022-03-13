use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInflationGovernorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

impl GetInflationGovernorRequest {
    pub fn new() -> Self {
        Self { commitment: None }
    }
    pub fn new_with_commitment(commitment: Commitment) -> Self {
        Self {
            commitment: Some(commitment),
        }
    }
}

impl Into<serde_json::Value> for GetInflationGovernorRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetInflationGovernorRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getInflationGovernor");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInflationGovernorResponse {
    initial: f64,
    terminal: f64,
    taper: f64,
    foundation: f64,
    foundation_term: f64,
}

impl From<RpcResponse> for GetInflationGovernorResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
