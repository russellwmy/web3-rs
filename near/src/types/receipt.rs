use super::{AccountId, Action, BlockHash, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptAction {
    pub actions: Vec<Action>,
    pub gas_price: String,
    pub input_data_ids: Vec<String>,
    pub output_data_receivers: Vec<String>,
    pub signer_id: AccountId,
    pub signer_public_key: PublicKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptInfo {
    #[serde(rename = "PascalCase")]
    pub action: ReceiptAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub predecessor_id: AccountId,
    pub receipt_id: BlockHash,
    pub receiver_id: AccountId,
    #[serde(alias = "receipt")]
    pub receipt_info: ReceiptInfo,
}
