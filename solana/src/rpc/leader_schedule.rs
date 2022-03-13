use {
    super::{
        serde_utils::{deserialize_public_key, serialize_public_key},
        types::Commitment,
    },
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
    std::collections::HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLeaderScheduleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_public_key")]
    pub identity: Option<Pubkey>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLeaderScheduleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<GetLeaderScheduleConfig>,
}

impl GetLeaderScheduleRequest {
    pub fn new() -> Self {
        Self {
            slot: None,
            config: None,
        }
    }
    pub fn new_with_slot_and_config(slot: u64, config: GetLeaderScheduleConfig) -> Self {
        Self {
            slot: Some(slot),
            config: Some(config),
        }
    }
    pub fn new_with_config(config: GetLeaderScheduleConfig) -> Self {
        Self {
            slot: None,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetLeaderScheduleRequest {
    fn into(self) -> serde_json::Value {
        match (self.slot, self.config) {
            (Some(slot), Some(config)) => serde_json::json!([slot, config]),
            (Some(slot), None) => serde_json::json!([slot]),
            (None, Some(config)) => serde_json::json!([config]),
            _ => serde_json::Value::Null,
        }
    }
}

impl Into<RpcRequest> for GetLeaderScheduleRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getLeaderSchedule");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderScheduleValue {
    lamports: u64,
    #[serde(deserialize_with = "deserialize_public_key")]
    address: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLeaderScheduleResponse(Option<HashMap<String, Vec<u64>>>);

impl From<RpcResponse> for GetLeaderScheduleResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
