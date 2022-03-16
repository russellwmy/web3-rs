use {
    super::{serde_utils::deserialize_public_key, types::Commitment, DataSlice, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProgramAccountsRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    pub encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSlice>,
    // TODO: Convert this to a struct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_context: Option<DataSlice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProgramAccountsRequest {
    pub public_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetProgramAccountsRequestConfig>,
}

impl GetProgramAccountsRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            config: None,
        }
    }

    pub fn new_with_config(public_key: Pubkey, config: GetProgramAccountsRequestConfig) -> Self {
        Self {
            public_key,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetProgramAccountsRequest {
    fn into(self) -> serde_json::Value {
        let public_key = self.public_key.to_string();

        match self.config {
            Some(config) => serde_json::json!([public_key, config]),
            None => serde_json::json!([public_key]),
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
    pub lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    pub owner: Pubkey,
    // TODO: Convert this to a struct
    pub data: serde_json::Value,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramAccountsValue {
    #[serde(deserialize_with = "deserialize_public_key")]
    pub pubkey: Pubkey,
    pub account: ProgramAccountsValueItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProgramAccountsResponse(Option<Vec<ProgramAccountsValue>>);

impl From<RpcResponse> for GetProgramAccountsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
