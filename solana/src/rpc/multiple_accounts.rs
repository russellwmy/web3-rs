use {
    super::{deserialize_public_key, types::Commitment, Context, DataSlice, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultipleAccountsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
    encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_slice: Option<DataSlice>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMultipleAccountsRequest {
    addresses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetMultipleAccountsConfig>,
}

impl GetMultipleAccountsRequest {
    pub fn new(addresses: Vec<String>) -> Self {
        Self {
            addresses,
            config: None,
        }
    }
    pub fn new_with_config(addresses: Vec<String>, config: GetMultipleAccountsConfig) -> Self {
        Self {
            addresses,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetMultipleAccountsRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([self.addresses, config]),
            None => serde_json::json!([self.addresses]),
        }
    }
}

impl Into<RpcRequest> for GetMultipleAccountsRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getMultipleAccounts");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleAccountsValue {
    lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    owner: Pubkey,
    data: serde_json::Value,
    executable: bool,
    rent_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMultipleAccountsResponse {
    context: Context,
    value: Option<Vec<MultipleAccountsValue>>,
}

impl From<RpcResponse> for GetMultipleAccountsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
