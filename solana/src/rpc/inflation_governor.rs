use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationGovernorRequestConfig {
    pub commitment: Commitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInflationGovernorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetInflationGovernorRequestConfig>,
}

impl GetInflationGovernorRequest {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn new_with_config(config: GetInflationGovernorRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetInflationGovernorRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config]),
            None => serde_json::Value::Null,
        }
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
    pub initial: f64,
    pub terminal: f64,
    pub taper: f64,
    pub foundation: f64,
    pub foundation_term: f64,
}

impl From<RpcResponse> for GetInflationGovernorResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
