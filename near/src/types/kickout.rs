use {super::AccountId, std::collections::HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kickout {
    pub account_id: AccountId,
    pub reason: HashMap<String, KickoutReason>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickoutReason {
    pub expected: u64,
    pub produced: u64,
}
