use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{
            AccessKey,
            AccountId,
            BlockHash,
            BlockHeight,
            BlockId,
            Finality,
            PublicKey,
            RequestType,
        },
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccessKeyListRequest {
    pub request_type: RequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub account_id: AccountId,
}

impl ViewAccessKeyListRequest {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            request_type: RequestType::ViewAccessKeyList,
            finality: None,
            block_id: None,
            account_id,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewAccessKeyListRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewAccessKeyListRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewAccessKeyListRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewAccessKeyListRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("query");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessKeyView {
    pub public_key: PublicKey,
    pub access_key: AccessKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccessKeyListResponse {
    pub keys: Vec<AccessKeyView>,
    pub block_height: BlockHeight,
    pub block_hash: BlockHash,
}

impl From<RpcResponse> for ViewAccessKeyListResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
