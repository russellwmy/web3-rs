use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccountChange, AccountId, BlockHash, BlockId, Changes, ChangesType, Finality},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccountChangesRequest {
    pub changes_type: ChangesType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub account_ids: Vec<AccountId>,
}

impl ViewAccountChangesRequest {
    pub fn new(account_ids: Vec<AccountId>) -> Self {
        Self {
            changes_type: ChangesType::AccountChanges,
            finality: None,
            block_id: None,
            account_ids,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewAccountChangesRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewAccountChangesRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewAccountChangesRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewAccountChangesRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("EXPERIMENTAL_changes");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewAccountChangesResponse {
    pub block_hash: BlockHash,
    pub changes: Vec<Changes<AccountChange>>,
}

impl From<RpcResponse> for ViewAccountChangesResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
