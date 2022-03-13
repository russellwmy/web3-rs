use crate::core::{RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEpochScheduleRequest {}

impl GetEpochScheduleRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetEpochScheduleRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetEpochScheduleRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getEpochSchedule");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEpochScheduleResponse {
    slots_per_epoch: u64,
    leader_schedule_slot_offset: u64,
    warmup: bool,
    first_normal_epoch: u64,
    first_normal_slot: u64,
}

impl From<RpcResponse> for GetEpochScheduleResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
