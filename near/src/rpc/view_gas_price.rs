use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::BlockId,
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewGasPriceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<Vec<BlockId>>,
}

impl ViewGasPriceRequest {
    pub fn new() -> Self {
        Self {
            block_height: None,
            block_id: None,
        }
    }

    pub fn block_height<'a>(&'a mut self, block_height: Vec<u64>) -> &'a mut ViewGasPriceRequest {
        self.block_height = Some(block_height);
        self
    }

    pub fn block_id<'a>(&'a mut self, block_id: Vec<BlockId>) -> &'a mut ViewGasPriceRequest {
        self.block_id = Some(block_id);
        self
    }
}

impl Into<serde_json::Value> for ViewGasPriceRequest {
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

impl Into<RpcRequest> for ViewGasPriceRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("gas_price");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewGasPriceResponse {
    pub gas_price: String,
}

impl From<RpcResponse> for ViewGasPriceResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
