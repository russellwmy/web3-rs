use {
    crate::{
        core::RpcRequest,
        types::{TransactionResult, TransactionString},
    },
    serde::Deserialize,
    web3_core::RpcResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionSyncRequest {
    pub transaction: TransactionString,
}

impl SendTransactionSyncRequest {
    pub fn new(transaction: TransactionString) -> Self {
        Self { transaction }
    }
}

impl Into<serde_json::Value> for SendTransactionSyncRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.transaction])
    }
}

impl Into<RpcRequest> for SendTransactionSyncRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("broadcast_tx_commit");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionSyncResponse(TransactionResult);

impl From<RpcResponse> for SendTransactionSyncResponse {
    fn from(response: RpcResponse) -> Self {
        let result = serde_json::from_value(response.result).unwrap();

        SendTransactionSyncResponse(result)
    }
}
