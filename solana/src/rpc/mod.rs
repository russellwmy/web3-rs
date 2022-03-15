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
mod max_retransmit_slot;
mod minimum_balance_for_rent_exemption;
mod multiple_accounts;
mod program_accounts;
mod recent_performance_samples;
mod serde_utils;
mod signature_statuses;
mod signatures_for_address;
mod slot;
mod slot_leader;
mod slot_leaders;
mod stake_activation;
mod supply;
mod types;

pub use {
    account_info::{AccountInfoValue, GetAccountInfoRequest, GetAccountInfoResponse},
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
    max_retransmit_slot::{GetMaxRetransmitSlotRequest, GetMaxRetransmitSlotResponse},
    minimum_balance_for_rent_exemption::{
        GetMinimumBalanceForRentExemptionRequest, GetMinimumBalanceForRentExemptionResponse,
    },
    multiple_accounts::{GetMultipleAccountsRequest, GetMultipleAccountsResponse},
    program_accounts::{GetProgramAccountsRequest, GetProgramAccountsResponse},
    recent_performance_samples::{
        GetRecentPerformanceSamplesRequest, GetRecentPerformanceSamplesResponse,
    },
    signature_statuses::{GetSignatureStatusesRequest, GetSignatureStatusesResponse},
    signatures_for_address::{GetSignaturesForAddressRequest, GetSignaturesForAddressResponse},
    slot::{GetSlotRequest, GetSlotResponse},
    slot_leader::{GetSlotLeaderRequest, GetSlotLeaderResponse},
    slot_leaders::{GetSlotLeadersRequest, GetSlotLeadersResponse},
    solana_sdk::clock::Slot,
    stake_activation::{GetStakeActivationRequest, GetStakeActivationResponse},
    supply::{GetSupplyRequest, GetSupplyResponse},
};

use serde::Deserialize;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSlice {
    pub offset: usize,
    pub length: usize,
}
