use std::str::FromStr;

use serde_json::from_str;

use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSupplyRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_non_circulating_accounts_list: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSupplyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetSupplyRequestConfig>,
}

impl GetSupplyRequest {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn new_with_config(config: GetSupplyRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetSupplyRequest {
    fn into(self) -> serde_json::Value {
        match self.config {
            Some(config) => serde_json::json!([config]),
            None => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetSupplyRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getSupply");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyResponseValue {
    total: u64,
    circulating: u64,
    non_circulating: u64,
    non_circulating_accounts: Vec<Pubkey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSupplyResponse {
    context: Context,
    value: SupplyResponseValue,
}

impl From<RpcResponse> for GetSupplyResponse {
    fn from(response: RpcResponse) -> Self {
        let context = response.result["context"].clone();
        let value = response.result["value"].clone();
        let total = value["total"].as_u64().expect("total is a u64");
        let circulating = value["circulating"].as_u64().expect("circulating is a u64");
        let non_circulating = value["nonCirculating"]
            .as_u64()
            .expect("non_circulating is a u64");
        let non_circulating_accounts = value["nonCirculatingAccounts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|account| {
                let account = account.as_str().expect("public key is a string");
                Pubkey::from_str(account).expect("public key is valid")
            })
            .collect::<Vec<Pubkey>>();

        let value = serde_json::json!({
            "context": context,
            "value": SupplyResponseValue {
                total,
                circulating,
                non_circulating,
                non_circulating_accounts,
            },
        });
        serde_json::from_value(value).unwrap()
    }
}
