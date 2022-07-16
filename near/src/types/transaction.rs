use super::{AccountId, Action, BlockHash, PublicKey};

pub type TransactionString = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub signer_id: AccountId,
    pub public_key: PublicKey,
    pub nonce: u64,
    pub receiver_id: AccountId,
    pub actions: Vec<Action>,
    // TODO:
    pub signature: String,
    pub hash: BlockHash,
}
