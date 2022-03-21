use {
    super::{types::Commitment, Context, DataSlice, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::{account::Account, pubkey::Pubkey},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    MintAccount,
    ProgramAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountsByDelegateRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    pub encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSlice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountsByDelegateRequest {
    pub public_key: Pubkey,
    pub account_type: AccountType,
    pub account_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetTokenAccountsByDelegateRequestConfig>,
}

impl GetTokenAccountsByDelegateRequest {
    pub fn new_mint(public_key: Pubkey, account_key: Pubkey) -> Self {
        Self {
            public_key,
            account_key,
            account_type: AccountType::MintAccount,
            config: None,
        }
    }

    pub fn new_mint_with_config(
        public_key: Pubkey,
        account_key: Pubkey,
        config: GetTokenAccountsByDelegateRequestConfig,
    ) -> Self {
        Self {
            public_key,
            account_key,
            account_type: AccountType::MintAccount,
            config: Some(config),
        }
    }

    pub fn new_program(public_key: Pubkey, account_key: Pubkey) -> Self {
        Self {
            public_key,
            account_key,
            account_type: AccountType::ProgramAccount,
            config: None,
        }
    }

    pub fn new_program_with_config(
        public_key: Pubkey,
        account_key: Pubkey,
        config: GetTokenAccountsByDelegateRequestConfig,
    ) -> Self {
        Self {
            public_key,
            account_key,
            account_type: AccountType::ProgramAccount,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetTokenAccountsByDelegateRequest {
    fn into(self) -> serde_json::Value {
        let public_key = self.public_key.to_string();
        let account_key = match self.account_type {
            AccountType::MintAccount => {
                serde_json::json!({"mint": self.account_key.to_string()})
            }
            AccountType::ProgramAccount => {
                serde_json::json!({"programId": self.account_key.to_string()})
            }
        };

        match self.config {
            Some(config) => serde_json::json!([public_key, account_key, config]),
            None => {
                let config = serde_json::json!({ "encoding": Encoding::JsonParsed });
                serde_json::json!([public_key, account_key, config])
            }
        }
    }
}

impl Into<RpcRequest> for GetTokenAccountsByDelegateRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getTokenAccountsByDelegate");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountsByDelegateResponseValue {
    #[serde(rename = "pubkey")]
    pub public_key: Pubkey,
    pub account: Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAccountsByDelegateResponse {
    pub context: Context,
    pub value: Option<Vec<TokenAccountsByDelegateResponseValue>>,
}

impl From<RpcResponse> for GetTokenAccountsByDelegateResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
