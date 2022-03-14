use solana_sdk::signature::Signature;

use {
    super::{serde_utils::deserialize_signature, types::Commitment},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignaturesForAddressConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignaturesForAddressRequest {
    public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetSignaturesForAddressConfig>,
}

impl GetSignaturesForAddressRequest {
    pub fn new(public_key: &str) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");
        Self {
            public_key,
            config: None,
        }
    }

    pub fn new_with_config(public_key: &str, config: GetSignaturesForAddressConfig) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");

        Self {
            public_key,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetSignaturesForAddressRequest {
    fn into(self) -> serde_json::Value {
        let public_key = self.public_key.to_string();

        match self.config {
            Some(config) => serde_json::json!([public_key, config]),
            None => serde_json::json!([public_key]),
        }
    }
}

impl Into<RpcRequest> for GetSignaturesForAddressRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getSignaturesForAddress");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignaturesForAddressValue {
    #[serde(deserialize_with = "deserialize_signature")]
    signature: Signature,
    slot: u64,
    err: Option<serde_json::Value>,
    memo: Option<String>,
    block_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignaturesForAddressResponse(Option<Vec<SignaturesForAddressValue>>);

impl From<RpcResponse> for GetSignaturesForAddressResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
