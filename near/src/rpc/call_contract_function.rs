use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{AccountId, BlockHash, BlockHeight, BlockId, Finality, RequestType},
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallContractFunctionRequest {
    pub request_type: RequestType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub method_name: String,
    pub args_base64: String,
    pub account_id: AccountId,
}

impl CallContractFunctionRequest {
    pub fn new(account_id: AccountId, method_name: &str, args_base64: &str) -> Self {
        Self {
            request_type: RequestType::CallContractFunction,
            finality: None,
            block_id: None,
            method_name: method_name.to_string(),
            args_base64: args_base64.to_string(),
            account_id,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut CallContractFunctionRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut CallContractFunctionRequest {
        self.block_id = block_id;
        self
    }
}

impl Into<serde_json::Value> for CallContractFunctionRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for CallContractFunctionRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("query");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallContractFunctionResponse {
    pub result: Vec<u64>,
    pub logs: Vec<String>,
    pub block_height: BlockHeight,
    pub block_hash: BlockHash,
}

impl From<RpcResponse> for CallContractFunctionResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
