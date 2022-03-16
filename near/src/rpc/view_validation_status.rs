use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{BlockId, ValidationStatus},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewValidationStatusRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<Vec<BlockId>>,
}

impl ViewValidationStatusRequest {
    pub fn new() -> Self {
        Self {
            block_height: None,
            block_id: None,
        }
    }

    pub fn block_height<'a>(
        &'a mut self,
        block_height: Vec<u64>,
    ) -> &'a mut ViewValidationStatusRequest {
        self.block_height = Some(block_height);
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Vec<BlockId>,
    ) -> &'a mut ViewValidationStatusRequest {
        self.block_id = Some(block_id);
        self
    }
}

impl Into<serde_json::Value> for ViewValidationStatusRequest {
    fn into(self) -> serde_json::Value {
        match self.block_height {
            Some(block_height) => {
                serde_json::json!(block_height)
            }
            None => match self.block_id {
                Some(block_id) => {
                    serde_json::json!(block_id)
                }
                None => {
                    serde_json::json!([serde_json::Value::Null])
                }
            },
        }
    }
}

impl Into<RpcRequest> for ViewValidationStatusRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("validators");
        let params = self.into();

        request.params(params).clone()
    }
}

pub type ViewValidationStatusResponse = ValidationStatus;

impl From<RpcResponse> for ViewValidationStatusResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
