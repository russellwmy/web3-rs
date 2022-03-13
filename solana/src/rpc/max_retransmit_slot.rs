use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMaxRetransmitSlotRequest {}

impl GetMaxRetransmitSlotRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetMaxRetransmitSlotRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetMaxRetransmitSlotRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getMaxRetransmitSlot");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMaxRetransmitSlotResponse(u64);

impl From<RpcResponse> for GetMaxRetransmitSlotResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
