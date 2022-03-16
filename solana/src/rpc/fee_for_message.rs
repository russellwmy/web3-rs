use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::message::Message,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeeForMessageRequestConfig {
    pub commitment: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeeForMessageRequest {
    pub message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetFeeForMessageRequestConfig>,
}

impl GetFeeForMessageRequest {
    pub fn new(message: Message) -> Self {
        Self {
            message,
            config: None,
        }
    }
    pub fn new_with_config(message: Message, config: GetFeeForMessageRequestConfig) -> Self {
        Self {
            message,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetFeeForMessageRequest {
    fn into(self) -> serde_json::Value {
        let message = base64::encode(self.message.serialize());

        match self.config {
            Some(config) => serde_json::json!([message, config]),
            None => serde_json::json!([message]),
        }
    }
}

impl Into<RpcRequest> for GetFeeForMessageRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getFeeForMessage");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeForMessageValue(Option<u64>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeeForMessageResponse {
    pub context: Context,
    pub value: FeeForMessageValue,
}

impl From<RpcResponse> for GetFeeForMessageResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
