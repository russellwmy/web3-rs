use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub id: u32,
    #[serde(rename(serialize = "jsonrpc", deserialize = "jsonrpc"))]
    pub jsonrpc_version: String,
    pub result: Value,
}
