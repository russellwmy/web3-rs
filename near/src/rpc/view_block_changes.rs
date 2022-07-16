use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{BlockChange, BlockHash, BlockId, Finality},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBlockChangesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
}

impl ViewBlockChangesRequest {
    pub fn new() -> Self {
        Self {
            finality: None,
            block_id: None,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewBlockChangesRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewBlockChangesRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewBlockChangesRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewBlockChangesRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("EXPERIMENTAL_changes_in_block");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBlockChangesResponse {
    pub block_hash: BlockHash,
    pub changes: Vec<BlockChange>,
}

impl From<RpcResponse> for ViewBlockChangesResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
