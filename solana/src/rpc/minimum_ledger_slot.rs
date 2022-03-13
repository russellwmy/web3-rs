use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumLedgerSlotRequest {}

impl MinimumLedgerSlotRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for MinimumLedgerSlotRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for MinimumLedgerSlotRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("minimumLedgerSlot");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumLedgerSlotResponse(u64);

impl From<RpcResponse> for MinimumLedgerSlotResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
