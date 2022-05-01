use {
    super::{serde_utils::deserialize_public_key, types::Commitment, Context, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfoRequestConfig {
    pub encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfoRequest {
    pub public_key: Pubkey,
    pub config: GetAccountInfoRequestConfig,
}

impl GetAccountInfoRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            config: GetAccountInfoRequestConfig {
                encoding: Encoding::Base58,
                commitment: None,
                data_slice: None,
            },
        }
    }
    pub fn new_with_config(public_key: Pubkey, config: GetAccountInfoRequestConfig) -> Self {
        Self { public_key, config }
    }
}

impl Into<serde_json::Value> for GetAccountInfoRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.public_key.to_string(), self.config])
    }
}

impl Into<RpcRequest> for GetAccountInfoRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getAccountInfo");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoValue {
    pub data: serde_json::Value,
    pub executable: bool,
    pub lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    pub owner: Pubkey,
    pub rent_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfoResponse {
    pub context: Context,
    pub value: AccountInfoValue,
}

impl From<RpcResponse> for GetAccountInfoResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
