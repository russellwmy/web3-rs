use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHighestSnapshotSlotRequest {}

impl GetHighestSnapshotSlotRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetHighestSnapshotSlotRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetHighestSnapshotSlotRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getHighestSnapshotSlot");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetHighestSnapshotSlotResponse {
    full: u64,
    incremental: Option<u64>,
}

impl From<RpcResponse> for GetHighestSnapshotSlotResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
