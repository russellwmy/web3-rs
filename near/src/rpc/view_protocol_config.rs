use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{BlockId, Finality, ProtocolConfig},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewProtocolConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
}

impl ViewProtocolConfigRequest {
    pub fn new() -> Self {
        Self {
            finality: None,
            block_id: None,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewProtocolConfigRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewProtocolConfigRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for ViewProtocolConfigRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewProtocolConfigRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("EXPERIMENTAL_protocol_config");
        let params = self.into();

        request.params(params).clone()
    }
}

pub type ViewProtocolConfigResponse = ProtocolConfig;

impl From<RpcResponse> for ViewProtocolConfigResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
