use {
    super::{serde_utils::deserialize_signature, types::Commitment},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignaturesForAddressRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignaturesForAddressRequest {
    public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetSignaturesForAddressRequestConfig>,
}

impl GetSignaturesForAddressRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            config: None,
        }
    }

    pub fn new_with_config(
        public_key: Pubkey,
        config: GetSignaturesForAddressRequestConfig,
    ) -> Self {
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
    pub signature: Signature,
    pub slot: u64,
    // TODO: Convert this to a struct
    pub err: Option<serde_json::Value>,
    pub memo: Option<String>,
    pub block_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignaturesForAddressResponse(Option<Vec<SignaturesForAddressValue>>);

impl From<RpcResponse> for GetSignaturesForAddressResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
