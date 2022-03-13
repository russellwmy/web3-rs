use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEpochInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

impl GetEpochInfoRequest {
    pub fn new() -> Self {
        Self { commitment: None }
    }
    pub fn new_with_commitment(commitment: Commitment) -> Self {
        Self {
            commitment: Some(commitment),
        }
    }
}

impl Into<serde_json::Value> for GetEpochInfoRequest {
    fn into(self) -> serde_json::Value {
        match self.commitment {
            Some(_) => serde_json::to_value([self]).unwrap(),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetEpochInfoRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getEpochInfo");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEpochInfoResponse {
    absolute_slot: u64,
    block_height: u64,
    epoch: u64,
    slot_index: u64,
    slots_in_epoch: u64,
    transaction_count: Option<u64>,
}

impl From<RpcResponse> for GetEpochInfoResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
