use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccountId, BlockHash, BlockHeight, BlockId, Finality, RequestType},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewContractStateRequest {
    pub request_type: RequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub prefix_base64: String,
    pub account_id: AccountId,
}

impl ViewContractStateRequest {
    pub fn new(account_id: AccountId) -> Self {
        Self {
            request_type: RequestType::ViewContractState,
            finality: None,
            block_id: None,
            prefix_base64: "".to_string(),
            account_id,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewContractStateRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewContractStateRequest {
        self.block_id = block_id;
        self
    }

    pub fn prefix_base64<'a>(
        &'a mut self,
        prefix_base64: String,
    ) -> &'a mut ViewContractStateRequest {
        self.prefix_base64 = prefix_base64;
        self
    }
}

impl Into<serde_json::Value> for ViewContractStateRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewContractStateRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("query");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStateValue {
    pub key: String,
    pub value: String,
    // TODO: convert this to a struct
    pub proof: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewContractStateResponse {
    pub values: Vec<ContractStateValue>,
    pub block_height: BlockHeight,
    pub block_hash: BlockHash,
    // TODO: convert this to a struct
    pub proof: Vec<serde_json::Value>,
}

impl From<RpcResponse> for ViewContractStateResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
