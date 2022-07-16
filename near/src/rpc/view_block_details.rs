use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{BlockDetails, BlockId, Chunk, Finality},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBlockDetailsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
}

impl ViewBlockDetailsRequest {
    pub fn new() -> Self {
        Self {
            finality: None,
            block_id: None,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewBlockDetailsRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewBlockDetailsRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewBlockDetailsRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewBlockDetailsRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("block");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewBlockDetailsResponse {
    pub author: String,
    pub header: BlockDetails,
    pub chunks: Vec<Chunk>,
}

impl From<RpcResponse> for ViewBlockDetailsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
