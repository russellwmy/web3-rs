use solana_sdk::hash::Hash;

mod account_info;
mod balance;
mod block_commitment;
mod block_height;
mod block_production;
mod block_time;
mod blocks;
mod blocks_with_limit;
mod cluster_nodes;
mod epoch_info;
mod epoch_schedule;
mod fee_for_message;
mod first_available_block;
mod genesis_hash;
mod health;
mod highest_snapshot_slot;
mod identity;
mod inflation_governor;
mod inflation_rate;
mod inflation_reward;
mod largest_accounts;
mod latest_blockhash;
mod leader_schedule;
mod types;

pub use {
    account_info::{GetAccountInfoRequest, GetAccountInfoRsponse},
    balance::{GetBalanceRequest, GetBalanceResponse},
    block_commitment::{GetBlockCommitmentRequest, GetBlockCommitmentResponse},
    block_height::{GetBlockHeightRequest, GetBlockHeightResponse},
    block_production::{GetBlockProductionRequest, GetBlockProductionResponse},
    block_time::{GetBlockTimeRequest, GetBlockTimeResponse},
    blocks::{GetBlocksRequest, GetBlocksResponse},
    blocks_with_limit::{GetBlocksWithLimitRequest, GetBlocksWithLimitResponse},
    cluster_nodes::{GetClusterNodesRequest, GetClusterNodesResponse},
    epoch_info::{GetEpochInfoRequest, GetEpochInfoResponse},
    epoch_schedule::{GetEpochScheduleRequest, GetEpochScheduleResponse},
    fee_for_message::{GetFeeForMessageRequest, GetFeeForMessageResponse},
    first_available_block::{GetFirstAvailableBlockRequest, GetFirstAvailableBlockResponse},
    genesis_hash::{GetGenesisHashRequest, GetGenesisHashResponse},
    health::{GetHealthRequest, GetHealthResponse},
    highest_snapshot_slot::{GetHighestSnapshotSlotRequest, GetHighestSnapshotSlotResponse},
    identity::{GetIdentityRequest, GetIdentityResponse},
    inflation_governor::{GetInflationGovernorRequest, GetInflationGovernorResponse},
    inflation_rate::{GetInflationRateRequest, GetInflationRateResponse},
    inflation_reward::{GetInflationRewardRequest, GetInflationRewardResponse},
    largest_accounts::{GetLargestAccountsRequest, GetLargestAccountsResponse},
    latest_blockhash::{GetLatestBlockhashRequest, GetLatestBlockhashResponse},
    leader_schedule::{
        GetLeaderScheduleConfig, GetLeaderScheduleRequest, GetLeaderScheduleResponse,
    },
    solana_sdk::clock::Slot,
};

use {
    serde::{Deserialize, Deserializer, Serializer},
    solana_sdk::pubkey::Pubkey,
    std::str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    JsonParsed,
    Base64,
    Base58,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    slot: Slot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockProductionRange {
    pub first_slot: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_slot: Option<u64>,
}

pub fn serialize_public_key<S>(public_key: &Option<Pubkey>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(public_key.unwrap().to_string().as_str())
}

pub fn deserialize_public_key<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(Pubkey::from_str(s.as_str()).unwrap())
}

pub fn deserialize_hash<'de, D>(deserializer: D) -> Result<Hash, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(Hash::from_str(s.as_str()).unwrap())
}
