use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    id: u32,
    #[serde(rename(serialize = "jsonrpc", deserialize = "jsonrpc"))]
    jsonrpc_version: String,
    method: String,
    params: Option<Value>,
}

impl RpcRequest {
    pub fn new(method: &str) -> Self {
        Self {
            id: 0,
            jsonrpc_version: "2.0".to_owned(),
            method: method.to_owned(),
            params: None,
        }
    }
    pub fn id<'a>(&'a mut self, id: u32) -> &'a mut RpcRequest {
        self.id = id;
        self
    }

    pub fn jsonrpc_version<'a>(&'a mut self, jsonrpc_version: &str) -> &'a mut RpcRequest {
        self.jsonrpc_version = jsonrpc_version.to_owned();
        self
    }

    pub fn params<'a>(&'a mut self, params: Value) -> &'a mut RpcRequest {
        self.params = Some(params);
        self
    }
}
