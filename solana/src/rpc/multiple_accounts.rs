use {
    super::{serde_utils::deserialize_public_key, types::Commitment, Context, DataSlice, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultipleAccountsRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    pub encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSlice>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMultipleAccountsRequest {
    pub addresses: Vec<Pubkey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetMultipleAccountsRequestConfig>,
}

impl GetMultipleAccountsRequest {
    pub fn new(addresses: Vec<Pubkey>) -> Self {
        Self {
            addresses,
            config: None,
        }
    }
    pub fn new_with_config(
        addresses: Vec<Pubkey>,
        config: GetMultipleAccountsRequestConfig,
    ) -> Self {
        Self {
            addresses,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetMultipleAccountsRequest {
    fn into(self) -> serde_json::Value {
        let addresses = self
            .addresses
            .iter()
            .map(|address| address.to_string())
            .collect::<Vec<String>>();

        match self.config {
            Some(config) => serde_json::json!([addresses, config]),
            None => serde_json::json!([addresses]),
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
    pub lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    pub owner: Pubkey,
    // TODO: Convert this to a struct
    pub data: serde_json::Value,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMultipleAccountsResponse {
    pub context: Context,
    pub value: Option<Vec<MultipleAccountsValue>>,
}

impl From<RpcResponse> for GetMultipleAccountsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
