use super::{AccountId, PublicKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountKey {
    pub account_id: AccountId,
    pub public_key: PublicKey,
}
