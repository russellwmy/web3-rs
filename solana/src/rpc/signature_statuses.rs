use {
    super::{types::Commitment, Context},
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::signature::Signature,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSignatureStatusesRequestConfig {
    pub search_transaction_history: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignatureStatusesRequest {
    pub signatures: Vec<Signature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<GetSignatureStatusesRequestConfig>,
}

impl GetSignatureStatusesRequest {
    pub fn new(signatures: Vec<Signature>) -> Self {
        Self {
            signatures,
            config: None,
        }
    }

    pub fn new_with_config(
        signatures: Vec<Signature>,
        config: GetSignatureStatusesRequestConfig,
    ) -> Self {
        Self {
            signatures,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for GetSignatureStatusesRequest {
    fn into(self) -> serde_json::Value {
        let signatures = self
            .signatures
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        match self.config {
            Some(config) => serde_json::json!([signatures, config]),
            None => serde_json::json!([signatures]),
        }
    }
}

impl Into<RpcRequest> for GetSignatureStatusesRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getSignatureStatuses");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureStatusesValue {
    pub slot: u64,
    pub confirmations: u64,
    // TODO: Convert this to a struct
    pub err: Option<serde_json::Value>,
    pub confirmation_status: Option<Commitment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignatureStatusesResponse {
    pub context: Context,
    pub value: Vec<Option<SignatureStatusesValue>>,
}

impl From<RpcResponse> for GetSignatureStatusesResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
