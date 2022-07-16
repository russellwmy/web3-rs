use super::{AccountId, BlockHash, Receipt, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionOutcomeStatus {
    pub success_receipt_id: Option<String>,
    pub success_value: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    hash: BlockHash,
    // TODO: convert this to a struct
    direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub logs: Vec<String>,
    // TODO: convert this to a struct
    pub receipt_ids: Vec<String>,
    pub gas_burnt: u64,
    pub tokens_burnt: String,
    pub executor_id: AccountId,
    pub status: TransactionOutcomeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutcome {
    pub proof: Vec<Proof>,
    pub id: String,
    pub block_hash: BlockHash,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub status: TransactionOutcomeStatus,
    pub transaction: Transaction,
    pub transaction_outcome: TransactionOutcome,
    pub receipts_outcome: Vec<TransactionOutcome>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResultWithReceipts {
    pub receipts: Vec<Receipt>,
    pub transaction: Transaction,
    pub transaction_outcome: TransactionOutcome,
    pub receipts_outcome: Vec<TransactionOutcome>,
}
