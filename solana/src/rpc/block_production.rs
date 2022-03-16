use {
    super::{types::Commitment, BlockProductionRange, Context},
    crate::core::{RpcRequest, RpcResponse},
    serde::Deserialize,
    solana_sdk::pubkey::Pubkey,
    std::collections::HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockProductionRequestConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<BlockProductionRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Pubkey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockProductionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetBlockProductionRequestConfig>,
}

impl GetBlockProductionRequest {
    pub fn new() -> Self {
        Self {
            config: Some(GetBlockProductionRequestConfig {
                commitment: None,
                range: None,
                identity: None,
            }),
        }
    }
    pub fn new_with_config(config: GetBlockProductionRequestConfig) -> Self {
        Self {
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetBlockProductionRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!([self.config])
    }
}

impl Into<RpcRequest> for GetBlockProductionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getBlockProduction");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockProductionValue {
    pub by_identity: HashMap<String, (usize, usize)>,
    pub range: BlockProductionRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockProductionResponse {
    pub context: Context,
    pub value: BlockProductionValue,
}

impl From<RpcResponse> for GetBlockProductionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
