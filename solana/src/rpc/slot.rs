use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotRequestConfig {
    commitment: Commitment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetSlotRequestConfig>,
}

impl GetSlotRequest {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn new_with_config(config: GetSlotRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetSlotRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config.commitment]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetSlotRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getSlot");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotResponse(u64);

impl From<RpcResponse> for GetSlotResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
