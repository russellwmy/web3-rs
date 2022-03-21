use {
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotLeadersRequest {
    pub start_slot: u64,
    pub limit: u64,
}

impl GetSlotLeadersRequest {
    pub fn new(start_slot: u64, limit: u64) -> Self {
        Self { start_slot, limit }
    }
}

impl Into<serde_json::Value> for GetSlotLeadersRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.start_slot, self.limit])
    }
}

impl Into<RpcRequest> for GetSlotLeadersRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getSlotLeaders");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSlotLeadersResponse(Vec<Pubkey>);

impl From<RpcResponse> for GetSlotLeadersResponse {
    fn from(response: RpcResponse) -> Self {
        let public_keys = response
            .result
            .as_array()
            .expect("public keys is an array")
            .to_vec();
        let public_keys = public_keys
            .iter()
            .map(|public_key| {
                let public_key = public_key.as_str().expect("public key is a string");
                Pubkey::from_str(public_key).expect("public key is valid")
            })
            .collect();
        GetSlotLeadersResponse(public_keys)
    }
}
