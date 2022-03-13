use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
    solana_sdk::{pubkey::Pubkey, signature::Signature},
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestAirdropConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestAirdropRequest {
    public_key: Pubkey,
    lamports: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<RequestAirdropConfig>,
}

impl RequestAirdropRequest {
    pub fn new(public_key: &str, lamports: u64) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");

        Self {
            public_key,
            lamports,
            config: None,
        }
    }
    pub fn new_with_config(public_key: &str, lamports: u64, config: RequestAirdropConfig) -> Self {
        let public_key = Pubkey::from_str(public_key).expect("invalid public key");

        Self {
            public_key,
            lamports,
            config: Some(config),
        }
    }
}

impl Into<serde_json::Value> for RequestAirdropRequest {
    fn into(self) -> serde_json::Value {
        let public_key = self.public_key.to_string();

        match self.config {
            Some(config) => serde_json::json!([public_key, self.lamports, config]),
            None => serde_json::json!([public_key, self.lamports]),
        }
    }
}

impl Into<RpcRequest> for RequestAirdropRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("requestAirdrop");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestAirdropResponse(Signature);

impl Into<Signature> for RequestAirdropResponse {
    fn into(self) -> Signature {
        self.0
    }
}

impl From<RpcResponse> for RequestAirdropResponse {
    fn from(response: RpcResponse) -> Self {
        let signature = response.result.as_str().expect("invalid response");
        let signature = Signature::from_str(signature).expect("invalid signature");

        RequestAirdropResponse(signature)
    }
}
