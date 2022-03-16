use super::{AccountId, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: PublicKey,
    pub addr: String,
    pub account_id: Option<AccountId>,
}
