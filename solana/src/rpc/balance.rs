use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

impl GetBalanceRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            commitment: None,
        }
    }
    pub fn new_with_commitment(public_key: Pubkey, commitment: Commitment) -> Self {
        Self {
            public_key,
            commitment: Some(commitment),
        }
    }
}

impl Into<serde_json::Value> for GetBalanceRequest {
    fn into(self) -> serde_json::Value {
        match self.commitment {
            Some(commitment) => serde_json::json!([self.public_key, commitment]),
            None => serde_json::json!([self.public_key.to_string()]),
        }
    }
}

impl Into<RpcRequest> for GetBalanceRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBalance");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    context: Context,
    value: u64,
}

impl From<RpcResponse> for GetBalanceResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
