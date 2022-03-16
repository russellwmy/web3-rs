use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccountId, TransactionResultWithReceipts},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewTransactionStatusWithReceiptsRequest {
    pub hash: String,
    pub account_id: AccountId,
}

impl ViewTransactionStatusWithReceiptsRequest {
    pub fn new(hash: String, account_id: AccountId) -> Self {
        Self { hash, account_id }
    }
}

impl Into<serde_json::Value> for ViewTransactionStatusWithReceiptsRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.hash, self.account_id])
    }
}

impl Into<RpcRequest> for ViewTransactionStatusWithReceiptsRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("EXPERIMENTAL_tx_status");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewTransactionStatusWithReceiptsResponse(TransactionResultWithReceipts);

impl From<RpcResponse> for ViewTransactionStatusWithReceiptsResponse {
    fn from(response: RpcResponse) -> Self {
        let result = serde_json::from_value(response.result).unwrap();

        ViewTransactionStatusWithReceiptsResponse(result)
    }
}
