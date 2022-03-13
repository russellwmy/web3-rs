use crate::{Provider, RpcRequest, RpcResponse, RpcResult};

#[derive(Clone)]
pub struct RcpClient {
    provider: Provider,
}

impl RcpClient {
    pub fn new(provider: Provider) -> Self {
        Self { provider }
    }

    pub async fn send(&self, request: RpcRequest) -> RpcResult<RpcResponse> {
        match self.provider.clone() {
            Provider::Http(provider) => provider.send(&request).await,
        }
    }
}
