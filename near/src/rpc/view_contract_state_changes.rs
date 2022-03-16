use {
    crate::{
        core::{RpcRequest, RpcResponse},
        types::{
            AccountId,
            BlockHash,
            BlockId,
            Changes,
            ChangesType,
            ContractStateChange,
            Finality,
        },
    },
    serde::Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewContractStateChangesRequest {
    pub changes_type: ChangesType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finality: Option<Finality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<BlockId>,
    pub account_ids: Vec<AccountId>,
    pub key_prefix_base64: String,
}

impl ViewContractStateChangesRequest {
    pub fn new(account_ids: Vec<AccountId>) -> Self {
        Self {
            changes_type: ChangesType::ContractStateChanges,
            finality: None,
            block_id: None,
            key_prefix_base64: "".to_string(),
            account_ids,
        }
    }

    pub fn finality<'a>(
        &'a mut self,
        finality: Option<Finality>,
    ) -> &'a mut ViewContractStateChangesRequest {
        self.finality = finality;
        self
    }

    pub fn block_id<'a>(
        &'a mut self,
        block_id: Option<BlockId>,
    ) -> &'a mut ViewContractStateChangesRequest {
        self.block_id = block_id;
        self
    }

    pub fn key_prefix_base64<'a>(
        &'a mut self,
        key_prefix_base64: String,
    ) -> &'a mut ViewContractStateChangesRequest {
        self.key_prefix_base64 = key_prefix_base64;
        self
    }
}

impl Into<serde_json::Value> for ViewContractStateChangesRequest {
    fn into(self) -> serde_json::Value {
        serde_json::json!(self)
    }
}

impl Into<RpcRequest> for ViewContractStateChangesRequest {
    fn into(self) -> RpcRequest {
        let mut request = RpcRequest::new("EXPERIMENTAL_changes");
        let params = self.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewContractStateChangesResponse {
    pub block_hash: BlockHash,
    pub changes: Vec<Changes<ContractStateChange>>,
}

impl From<RpcResponse> for ViewContractStateChangesResponse {
    fn from(response: RpcResponse) -> Self {
        serde_json::from_value(response.result).unwrap()
    }
}
