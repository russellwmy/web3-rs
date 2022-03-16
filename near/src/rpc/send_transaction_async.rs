use {
    crate::{core::RpcRequest, types::TransactionString},
    serde::Deserialize,
    web3_core::RpcResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionAsyncRequest {
    pub transaction: TransactionString,
}

impl SendTransactionAsyncRequest {
    pub fn new(transaction: TransactionString) -> Self {
        Self { transaction }
    }
}

impl Into<serde_json::Value> for SendTransactionAsyncRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.transaction])
    }
}

impl Into<RpcRequest> for SendTransactionAsyncRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("broadcast_tx_async");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionAsyncResponse(String);

impl From<RpcResponse> for SendTransactionAsyncResponse {
    fn from(response: RpcResponse) -> Self {
        SendTransactionAsyncResponse(response.result.to_string())
    }
}
