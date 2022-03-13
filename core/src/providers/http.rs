use crate::{RpcError, RpcRequest, RpcResponse, RpcResult};

#[derive(Clone)]
pub struct HttpProvider {
    client: reqwest::Client,
    headers: reqwest::header::HeaderMap,
    url: String,
}

impl HttpProvider {
    pub fn new(url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            headers: reqwest::header::HeaderMap::new(),
            url,
        }
    }
}

impl HttpProvider {
    pub async fn send(self, request: &RpcRequest) -> RpcResult<RpcResponse> {
        let client = &self.client;
        let url = self.url.clone();
        let headers = self.headers.clone();

        let request_result: serde_json::Value = client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await
            .map_err(|e| RpcError::from(e))?
            .json()
            .await
            .map_err(|e| RpcError::from(e))?;

        match serde_json::from_value::<RpcResponse>(request_result.clone()) {
            Ok(response) => Ok(response),
            Err(_) => Err(serde_json::from_value::<RpcError>(request_result).unwrap()),
        }
    }
}
