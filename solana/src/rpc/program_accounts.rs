use {
    super::{deserialize_public_key, types::Commitment, DataSlice, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProgramAccountsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
    encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_slice: Option<DataSlice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_context: Option<DataSlice>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProgramAccountsRequest {
    public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetProgramAccountsConfig>,
}

impl GetProgramAccountsRequest {
    pub fn new(public_key: &str) -> Self {
        Self {
            public_key: public_key.to_owned(),
            config: None,
        }
    }
    pub fn new_with_config(public_key: &str, config: GetProgramAccountsConfig) -> Self {
        Self {
            public_key: public_key.to_owned(),
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetProgramAccountsRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([self.public_key, config]),
            None => serde_json::json!([self.public_key]),
        }
    }
}

impl Into<RpcRequest> for GetProgramAccountsRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getProgramAccounts");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAccountsValueItem {
    lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    owner: Pubkey,
    data: serde_json::Value,
    executable: bool,
    rent_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramAccountsValue {
    #[serde(deserialize_with = "deserialize_public_key")]
    pubkey: Pubkey,
    account: ProgramAccountsValueItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProgramAccountsResponse(Option<Vec<ProgramAccountsValue>>);

impl From<RpcResponse> for GetProgramAccountsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
