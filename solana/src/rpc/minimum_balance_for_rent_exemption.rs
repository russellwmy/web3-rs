use {
    super::types::Commitment,
    crate::core::{RpcRequest, RpcResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinimumBalanceForRentExemptionRequest {
    data_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<Commitment>,
}

impl GetMinimumBalanceForRentExemptionRequest {
    pub fn new(data_length: usize) -> Self {
        Self {
            data_length,
            commitment: None,
        }
    }
    pub fn new_with_commitment(data_length: usize, commitment: Commitment) -> Self {
        Self {
            data_length,
            commitment: Some(commitment),
        }
    }
}

impl Into<serde_json::Value> for GetMinimumBalanceForRentExemptionRequest {
    fn into(self) -> serde_json::Value {
        match self.commitment {
            Some(commitment) => {
                serde_json::json!([self.data_length, {"commitment": commitment.to_string()}])
            }
            None => serde_json::json!([self.data_length]),
        }
    }
}

impl Into<RpcRequest> for GetMinimumBalanceForRentExemptionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("getMinimumBalanceForRentExemption");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinimumBalanceForRentExemptionResponse(u64);

impl From<RpcResponse> for GetMinimumBalanceForRentExemptionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
