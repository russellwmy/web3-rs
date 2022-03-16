use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccountId, BlockHash, BlockHeight, BlockId, Finality, RequestType},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccountRequest {
    pub request_type: RequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub account_id: AccountId,
}

impl ViewAccountRequest {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            request_type: RequestType::ViewAccount,
            finality: None,
            block_id: None,
            account_id,
        }
    }

    pub fn finality<'a>(&'a mut self, finality: Option<Finality>) -> &'a mut ViewAccountRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(&'a mut self, block_id: Option<BlockId>) -> &'a mut ViewAccountRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewAccountRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewAccountRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("query");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccountResponse {
    pub amount: String,
    pub locked: String,
    pub code_hash: String,
    pub storage_usage: u64,
    pub storage_paid_at: u64,
    pub block_height: BlockHeight,
    pub block_hash: BlockHash,
}

impl From<RpcResponse> for ViewAccountResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
