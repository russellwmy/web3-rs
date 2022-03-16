use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAccountBalanceRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAccountBalanceRequest {
    pub account: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetTokenAccountBalanceRequestConfig>,
}

impl GetTokenAccountBalanceRequest {
    pub fn new(account: Pubkey) -> Self {
        Self {
            account,
            config: None,
        }
    }

    pub fn new_with_config(account: Pubkey, config: GetTokenAccountBalanceRequestConfig) -> Self {
        Self {
            account,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetTokenAccountBalanceRequest {
    fn into(self) -> serde_json::Value {
        let account = self.account.to_string();

        match self.config {
            Some(config) => serde_json::json!([account, config]),
            None => serde_json::json!([account]),
        }
    }
}

impl Into<RpcRequest> for GetTokenAccountBalanceRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getTokenAccountBalance");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountBalanceResponseValue {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<u64>,
    pub ui_amount_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAccountBalanceResponse {
    pub context: Context,
    pub value: TokenAccountBalanceResponseValue,
}

impl From<RpcResponse> for GetTokenAccountBalanceResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
