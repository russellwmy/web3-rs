use solana_sdk::account::Account;

use {
    super::{types::Commitment, Context, DataSlice, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    MintAccount,
    ProgramAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountsByDelegateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
    encoding: Encoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_slice: Option<DataSlice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountsByDelegateRequest {
    public_key: Pubkey,
    account_type: AccountType,
    account_key: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetTokenAccountsByDelegateConfig>,
}

impl GetTokenAccountsByDelegateRequest {
    pub fn new_mint(public_key: &str, mint_account_key: &str) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");
        let account_key = Pubkey::from_str(mint_account_key).expect("invalid public key");

        Self {
            public_key,
            account_key,
            account_type: AccountType::MintAccount,
            config: None,
        }
    }

    pub fn new_mint_with_config(
        public_key: &str,
        mint_account_key: &str,
        config: GetTokenAccountsByDelegateConfig,
    ) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");
        let account_key = Pubkey::from_str(mint_account_key).expect("invalid public key");

        Self {
            public_key,
            account_key,
            account_type: AccountType::MintAccount,
            config: Some(config),
        }
    }

    pub fn new_program(public_key: &str, program_account_key: &str) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");
        let account_key = Pubkey::from_str(program_account_key).expect("invalid public key");

        Self {
            public_key,
            account_key,
            account_type: AccountType::ProgramAccount,
            config: None,
        }
    }

    pub fn new_program_with_config(
        public_key: &str,
        program_account_key: &str,
        config: GetTokenAccountsByDelegateConfig,
    ) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");
        let account_key = Pubkey::from_str(program_account_key).expect("invalid public key");

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
    public_key: Pubkey,
    account: Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTokenAccountsByDelegateResponse {
    context: Context,
    value: Option<Vec<TokenAccountsByDelegateResponseValue>>,
}

impl From<RpcResponse> for GetTokenAccountsByDelegateResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
