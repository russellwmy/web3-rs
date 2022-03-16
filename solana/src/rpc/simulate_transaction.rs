use {
    super::{types::Commitment, Encoding},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::signature::Signature,
    solana_sdk::transaction::Transaction,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateTransactionConfig {
    pub skip_preflight: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preflight_commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<usize>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateTransactionRequest {
    transaction: Transaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<SimulateTransactionConfig>,
}

impl SimulateTransactionRequest {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            config: None,
        }
    }
    pub fn new_with_config(transaction: Transaction, config: SimulateTransactionConfig) -> Self {
        Self {
            transaction,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for SimulateTransactionRequest {
    fn into(self) -> serde_json::Value {
        let transaction_data = bincode::serialize(&self.transaction).unwrap();
        let encoded_transaction = bs58::encode(&transaction_data).into_string();

        match self.config {
            Some(config) => serde_json::json!([encoded_transaction, config]),
            None => serde_json::json!([encoded_transaction]),
        }
    }
}

impl Into<RpcRequest> for SimulateTransactionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("simulateTransaction");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateTransactionResponse(Signature);

impl From<RpcResponse> for SimulateTransactionResponse {
    fn from(response: RpcResponse) -> Self {
        let signature = response.result.as_str().expect("invalid response");
        let signature = Signature::from_str(signature).expect("invalid signature");

        SimulateTransactionResponse(signature)
    }
}
