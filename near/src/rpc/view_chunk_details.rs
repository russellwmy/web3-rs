use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{BlockId, ChunkDetails},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewChunkDetailsRequest {
    // TODO: convert this to a type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    // TODO: convert this to a type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<u64>,
}

impl ViewChunkDetailsRequest {
    pub fn new() -> Self {
        Self {
            chunk_id: None,
            block_id: None,
            shard_id: None,
        }
    }

    pub fn chunk_id<'a>(&'a mut self, chunk_id: Option<String>) -> &'a mut ViewChunkDetailsRequest {
        self.chunk_id = chunk_id;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewChunkDetailsRequest {
        self.block_id = block_id;
        self
    }

    pub fn shard_id<'a>(&'a mut self, shard_id: Option<u64>) -> &'a mut ViewChunkDetailsRequest {
        self.shard_id = shard_id;
        self
    }
}

impl Into<serde_json::Value> for ViewChunkDetailsRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewChunkDetailsRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("chunk");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewChunkDetailsResponse {
    pub author: String,
    pub header: ChunkDetails,
    // TODO: convert this to a type
    pub transactions: Vec<serde_json::Value>,
    // TODO: convert this to a type
    pub receipts: Vec<serde_json::Value>,
}

impl From<RpcResponse> for ViewChunkDetailsResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
