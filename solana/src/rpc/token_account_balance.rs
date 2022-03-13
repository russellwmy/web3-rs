use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountBalanceRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountBalanceRequest {
    account: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetTokenAccountBalanceRequestConfig>,
}

impl GetTokenAccountBalanceRequest {
    pub fn new(account: &str) -> Self {
        let account = Pubkey::from_str(account).expect("invalid public key");

        Self {
            account,
            config: None,
        }
    }

    pub fn new_with_config(account: &str, config: GetTokenAccountBalanceRequestConfig) -> Self {
        let account = Pubkey::from_str(account).expect("invalid public key");

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
    amount: String,
    decimals: u8,
    ui_amount: Option<u64>,
    ui_amount_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAccountBalanceResponse {
    context: Context,
    value: TokenAccountBalanceResponseValue,
}

impl From<RpcResponse> for GetTokenAccountBalanceResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
