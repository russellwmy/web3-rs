use {
    super::{types::Commitment, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::{signature::Signature, transaction::Transaction},
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionRequestConfig {
    pub skip_preflight: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preflight_commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<usize>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionRequest {
    transaction: Transaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<SendTransactionRequestConfig>,
}

impl SendTransactionRequest {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            config: None,
        }
    }
    pub fn new_with_config(transaction: Transaction, config: SendTransactionRequestConfig) -> Self {
        Self {
            transaction,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for SendTransactionRequest {
    fn into(self) -> serde_json::Value {
        let transaction_data = bincode::serialize(&self.transaction).unwrap();
        let encoded_transaction = bs58::encode(&transaction_data).into_string();

        match self.config {
            Some(config) => serde_json::json!([encoded_transaction, config]),
            None => serde_json::json!([encoded_transaction]),
        }
    }
}

impl Into<RpcRequest> for SendTransactionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("sendTransaction");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionResponse(Signature);

impl Into<Signature> for SendTransactionResponse {
    fn into(self) -> Signature {
        self.0
    }
}

impl From<RpcResponse> for SendTransactionResponse {
    fn from(response: RpcResponse) -> Self {
        let signature = response.result.as_str().expect("invalid response");
        let signature = Signature::from_str(signature).expect("invalid signature");

        SendTransactionResponse(signature)
    }
}
