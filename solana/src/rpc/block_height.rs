use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockHeightRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

impl GetBlockHeightRequest {
    pub fn new() -> Self {
        Self { commitment: None }
    }
    pub fn new_with_commitment(commitment: Commitment) -> Self {
        Self {
            commitment: Some(commitment),
        }
    }
}

impl Into<serde_json::Value> for GetBlockHeightRequest {
    fn into(self) -> serde_json::Value {
        match self.commitment {
            Some(_) => serde_json::json!([self]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetBlockHeightRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBlockHeight");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockHeightResponse(u64);

impl From<RpcResponse> for GetBlockHeightResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
