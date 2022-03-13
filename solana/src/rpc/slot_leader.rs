use std::str::FromStr;

use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotLeaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

impl GetSlotLeaderRequest {
    pub fn new() -> Self {
        Self { commitment: None }
    }

    pub fn new_with_commitment(commitment: Commitment) -> Self {
        Self {
            commitment: Some(commitment),
        }
    }
}

impl Into<serde_json::Value> for GetSlotLeaderRequest {
    fn into(self) -> serde_json::Value {
        match self.commitment {
            Some(commitment) => serde_json::json!([commitment]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetSlotLeaderRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getSlotLeader");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotLeaderResponse(Pubkey);

impl From<RpcResponse> for GetSlotLeaderResponse {
    fn from(response: RpcResponse) -> Self {
        let public_key = response.result.as_str().expect("public key is a string");
        GetSlotLeaderResponse(Pubkey::from_str(public_key).expect("public key is valid"))
    }
}
