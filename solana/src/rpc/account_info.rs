use {
    super::{deserialize_public_key, types::Commitment, Context, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoRequestConfig {
    pub encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfoRequest {
    public_key: Pubkey,
    config: AccountInfoRequestConfig,
}

impl GetAccountInfoRequest {
    pub fn new(public_key: Pubkey) -> Self {
        Self {
            public_key,
            config: AccountInfoRequestConfig {
                encoding: Encoding::Base58,
                commitment: None,
                data_slice: None,
            },
        }
    }
    pub fn new_with_config(public_key: Pubkey, config: AccountInfoRequestConfig) -> Self {
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
    data: serde_json::Value,
    executable: bool,
    lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    owner: Pubkey,
    rent_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfoRsponse {
    context: Context,
    value: AccountInfoValue,
}

impl From<RpcResponse> for GetAccountInfoRsponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
