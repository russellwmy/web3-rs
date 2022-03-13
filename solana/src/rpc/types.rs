use crate::constants::MAX_LOCKOUT_HISTORY;

pub type BlockCommitmentArray = [u64; MAX_LOCKOUT_HISTORY + 1];
pub type Commitment = solana_sdk::commitment_config::CommitmentLevel;
