use {
    super::types::BlockCommitmentArray,
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockCommitmentRequest {
    pub slot: u64,
}

impl GetBlockCommitmentRequest {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }
}

impl Into<serde_json::Value> for GetBlockCommitmentRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.slot])
    }
}

impl Into<RpcRequest> for GetBlockCommitmentRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBlockCommitment");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockCommitmentResponse {
    pub commitment: Option<BlockCommitmentArray>,
    pub total_stake: u64,
}

impl From<RpcResponse> for GetBlockCommitmentResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
