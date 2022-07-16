use super::Balance;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAction {
    pub deposit: Balance,
}

impl From<TransferAction> for Action {
    fn from(transfer_action: TransferAction) -> Self {
        Self::Transfer(transfer_action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallAction {
    pub args: String,
    pub deposit: Balance,
    pub gas: u64,
    pub method_name: String,
}

impl From<FunctionCallAction> for Action {
    fn from(function_call_action: FunctionCallAction) -> Self {
        Self::FunctionCall(function_call_action)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Transfer(TransferAction),
    FunctionCall(FunctionCallAction),
}
