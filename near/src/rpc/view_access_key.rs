use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccessKey, AccountId, BlockId, Finality, PublicKey, RequestType},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccountKeyRequest {
    pub request_type: RequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub public_key: PublicKey,
    pub account_id: AccountId,
}

impl ViewAccountKeyRequest {
    pub fn new(account_id: AccountId, public_key: PublicKey) -> Self {
        Self {
            request_type: RequestType::ViewAccessKey,
            finality: None,
            block_id: None,
            public_key,
            account_id,
        }
    }

    pub fn finality<'a>(&'a mut self, finality: Option<Finality>) -> &'a mut ViewAccountKeyRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(&'a mut self, block_id: Option<BlockId>) -> &'a mut ViewAccountKeyRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewAccountKeyRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewAccountKeyRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("query");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccountKeyResponse(AccessKey);

impl From<RpcResponse> for ViewAccountKeyResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
