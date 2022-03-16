use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccountId, BlockHash, BlockHeight, BlockId, Finality, RequestType},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewContractCodeRequest {
    pub request_type: RequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub account_id: AccountId,
}

impl ViewContractCodeRequest {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            request_type: RequestType::ViewContractCode,
            finality: None,
            block_id: None,
            account_id,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewContractCodeRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewContractCodeRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewContractCodeRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewContractCodeRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("query");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewContractCodeResponse {
    pub code_base64: String,
    pub hash: String,
    pub block_height: BlockHeight,
    pub block_hash: BlockHash,
}

impl From<RpcResponse> for ViewContractCodeResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
