#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    id: u32,
    #[serde(rename(serialize = "jsonrpc", deserialize = "jsonrpc"))]
    jsonrpc_version: String,
    error: Error,
}

impl From<reqwest::Error> for RpcError {
    fn from(error: reqwest::Error) -> Self {
        RpcError {
            id: 0,
            jsonrpc_version: "2.0".to_owned(),
            error: Error {
                code: error
                    .status()
                    .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
                    .as_u16() as i32,
                message: error.to_string(),
            },
        }
    }
}
