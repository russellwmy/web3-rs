use super::BlockHash;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangesCauseType {
    TransactionProcessing,
    ReceiptProcessing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangesCause {
    #[serde(alias = "type")]
    cause_type: ChangesCauseType,
    #[serde(alias = "tx_hash", alias = "receipt_hash")]
    hash: BlockHash,
}
