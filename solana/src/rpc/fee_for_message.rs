use super::Context;

use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeForMessageRequest {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

impl GetFeeForMessageRequest {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            commitment: None,
        }
    }
    pub fn new_with_commitment(message: &str, commitment: Commitment) -> Self {
        Self {
            message: message.to_owned(),
            commitment: Some(commitment),
        }
    }
}

impl Into<serde_json::Value> for GetFeeForMessageRequest {
    fn into(self) -> serde_json::Value {
        match self.commitment {
            Some(commitment) => {
                let commitment = commitment.to_string();

                serde_json::to_value([self.message, { commitment }]).unwrap()
            }
            None => serde_json::to_value([self.message]).unwrap(),
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
    context: Context,
    value: FeeForMessageValue,
}

impl From<RpcResponse> for GetFeeForMessageResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
