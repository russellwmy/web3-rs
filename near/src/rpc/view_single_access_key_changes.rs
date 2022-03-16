use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccessKeyChange, AccountKey, BlockHash, BlockId, Changes, ChangesType, Finality},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSingleAccessKeyChangesRequest {
    pub changes_type: ChangesType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub keys: Vec<AccountKey>,
}

impl ViewSingleAccessKeyChangesRequest {
    pub fn new(keys: Vec<AccountKey>) -> Self {
        Self {
            changes_type: ChangesType::SingleAccessKeyChanges,
            keys,
            block_id: None,
            finality: None,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewSingleAccessKeyChangesRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewSingleAccessKeyChangesRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewSingleAccessKeyChangesRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewSingleAccessKeyChangesRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("EXPERIMENTAL_changes");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSingleAccessKeyChangesResponse {
    pub block_hash: BlockHash,
    pub changes: Vec<Changes<AccessKeyChange>>,
}

impl From<RpcResponse> for ViewSingleAccessKeyChangesResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
