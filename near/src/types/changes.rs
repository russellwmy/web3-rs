use super::{AccessKey, AccountId, ChangesCause, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangesEvent {
    AccessKeyUpdate,
    AccessKeyTouched,
    AccountTouched,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Changes<T> {
    cause: ChangesCause,
    #[serde(alias = "type")]
    change_event: ChangesEvent,
    change: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessKeyChange {
    account_id: AccountId,
    public_key: PublicKey,
    access_key: AccessKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountChange {
    account_id: AccountId,
    amount: String,
    locked: String,
    code_hash: String,
    storage_usage: u64,
    storage_paid_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractStateChange {
    account_id: AccountId,
    key_base64: String,
    value_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCodeChange {
    account_id: AccountId,
    code_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockChange {
    #[serde(alias = "type")]
    change_event: ChangesEvent,
    account_id: AccountId,
}
