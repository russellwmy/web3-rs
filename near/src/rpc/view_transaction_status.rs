use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccountId, TransactionResult},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewTransactionStatusRequest {
    pub hash: String,
    pub account_id: AccountId,
}

impl ViewTransactionStatusRequest {
    pub fn new(hash: String, account_id: AccountId) -> Self {
        Self { hash, account_id }
    }
}

impl Into<serde_json::Value> for ViewTransactionStatusRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.hash, self.account_id])
    }
}

impl Into<RpcRequest> for ViewTransactionStatusRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("tx");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewTransactionStatusResponse(TransactionResult);

impl From<RpcResponse> for ViewTransactionStatusResponse {
    fn from(response: RpcResponse) -> Self {
        let result = serde_json::from_value(response.result).unwrap();

        ViewTransactionStatusResponse(result)
    }
}
