use super::{AccountId, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producer {
    pub addr: Option<String>,
    pub account_id: AccountId,
    pub peer_id: PublicKey,
}
