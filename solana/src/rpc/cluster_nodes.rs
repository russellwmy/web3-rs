use {
    super::serde_utils::deserialize_public_key,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetClusterNodesRequest {}

impl GetClusterNodesRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<serde_json::Value> for GetClusterNodesRequest {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl Into<RpcRequest> for GetClusterNodesRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getClusterNodes");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNode {
    #[serde(deserialize_with = "deserialize_public_key", rename = "pubkey")]
    pub public_key: Pubkey,
    pub gossip: Option<String>,
    pub tpu: Option<String>,
    pub rpc: Option<String>,
    pub version: Option<String>,
    pub feature_set: Option<u32>,
    pub shred_version: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetClusterNodesResponse(Vec<ClusterNode>);

impl From<RpcResponse> for GetClusterNodesResponse {
    fn from(response: RpcResponse) -> Self {
        let nodes = response
            .result
            .as_array()
            .unwrap()
            .iter()
            .map(|item| serde_json::from_value(item.clone()).unwrap())
            .collect::<Vec<ClusterNode>>();

        GetClusterNodesResponse(nodes)
    }
}
