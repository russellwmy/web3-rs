use {
    super::serde_utils::deserialize_public_key,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIdentityRequest {}

impl GetIdentityRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetIdentityRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetIdentityRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getIdentity");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityResponse {
    #[serde(deserialize_with = "deserialize_public_key")]
    identity: Pubkey,
}

impl From<RpcResponse> for GetIdentityResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
