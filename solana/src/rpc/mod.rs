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
mod minimum_ledger_slot;
mod multiple_accounts;
mod program_accounts;
mod recent_performance_samples;
mod request_airdrop;
mod send_transaction;
mod serde_utils;
mod signature_statuses;
mod signatures_for_address;
mod simulate_transaction;
mod slot;
mod slot_leader;
mod slot_leaders;
mod stake_activation;
mod supply;
mod token_account_balance;
mod token_accounts_by_delegate;
mod token_accounts_by_owner;
mod token_largest_accounts;
mod token_supply;
mod transaction;
mod types;
mod validate_blockhash;
mod version;
mod vote_accounts;

use serde::Deserialize;
pub use {
    account_info::{GetAccountInfoRequest, GetAccountInfoRequestConfig, GetAccountInfoResponse},
    balance::{GetBalanceRequest, GetBalanceRequestConfig, GetBalanceResponse},
    block_commitment::{GetBlockCommitmentRequest, GetBlockCommitmentResponse},
    block_height::{GetBlockHeightRequest, GetBlockHeightRequestConfig, GetBlockHeightResponse},
    block_production::{
        GetBlockProductionRequest,
        GetBlockProductionRequestConfig,
        GetBlockProductionResponse,
    },
    block_time::{GetBlockTimeRequest, GetBlockTimeResponse},
    blocks::{GetBlocksRequest, GetBlocksRequestConfig, GetBlocksResponse},
    blocks_with_limit::{
        GetBlocksWithLimitRequest,
        GetBlocksWithLimitRequestConfig,
        GetBlocksWithLimitResponse,
    },
    cluster_nodes::{GetClusterNodesRequest, GetClusterNodesResponse},
    epoch_info::{GetEpochInfoRequest, GetEpochInfoRequestConfig, GetEpochInfoResponse},
    epoch_schedule::{GetEpochScheduleRequest, GetEpochScheduleResponse},
    fee_for_message::{
        GetFeeForMessageRequest,
        GetFeeForMessageRequestConfig,
        GetFeeForMessageResponse,
    },
    first_available_block::{GetFirstAvailableBlockRequest, GetFirstAvailableBlockResponse},
    genesis_hash::{GetGenesisHashRequest, GetGenesisHashResponse},
    health::{GetHealthRequest, GetHealthResponse},
    highest_snapshot_slot::{GetHighestSnapshotSlotRequest, GetHighestSnapshotSlotResponse},
    identity::{GetIdentityRequest, GetIdentityResponse},
    inflation_governor::{
        GetInflationGovernorRequest,
        GetInflationGovernorRequestConfig,
        GetInflationGovernorResponse,
    },
    inflation_rate::{GetInflationRateRequest, GetInflationRateResponse},
    inflation_reward::{
        GetInflationRewardRequest,
        GetInflationRewardRequestConfig,
        GetInflationRewardResponse,
    },
    largest_accounts::{
        GetLargestAccountsRequest,
        GetLargestAccountsRequestConfig,
        GetLargestAccountsResponse,
    },
    latest_blockhash::{
        GetLatestBlockhashRequest,
        GetLatestBlockhashRequestConfig,
        GetLatestBlockhashResponse,
    },
    leader_schedule::{
        GetLeaderScheduleRequest,
        GetLeaderScheduleRequestConfig,
        GetLeaderScheduleResponse,
    },
    max_retransmit_slot::{GetMaxRetransmitSlotRequest, GetMaxRetransmitSlotResponse},
    minimum_balance_for_rent_exemption::{
        GetMinimumBalanceForRentExemptionRequest,
        GetMinimumBalanceForRentExemptionRequestConfig,
        GetMinimumBalanceForRentExemptionResponse,
    },
    minimum_ledger_slot::{MinimumLedgerSlotRequest, MinimumLedgerSlotResponse},
    multiple_accounts::{
        GetMultipleAccountsRequest,
        GetMultipleAccountsRequestConfig,
        GetMultipleAccountsResponse,
    },
    program_accounts::{
        GetProgramAccountsRequest,
        GetProgramAccountsRequestConfig,
        GetProgramAccountsResponse,
    },
    recent_performance_samples::{
        GetRecentPerformanceSamplesRequest,
        GetRecentPerformanceSamplesRequestConfig,
        GetRecentPerformanceSamplesResponse,
    },
    request_airdrop::{RequestAirdropRequest, RequestAirdropRequestConfig, RequestAirdropResponse},
    send_transaction::{
        SendTransactionRequest,
        SendTransactionRequestConfig,
        SendTransactionResponse,
    },
    signature_statuses::{
        GetSignatureStatusesRequest,
        GetSignatureStatusesRequestConfig,
        GetSignatureStatusesResponse,
    },
    signatures_for_address::{
        GetSignaturesForAddressRequest,
        GetSignaturesForAddressRequestConfig,
        GetSignaturesForAddressResponse,
    },
    simulate_transaction::{
        SimulateTransactionRequest,
        SimulateTransactionRequestConfig,
        SimulateTransactionResponse,
    },
    slot::{GetSlotRequest, GetSlotRequestConfig, GetSlotResponse},
    slot_leader::{GetSlotLeaderRequest, GetSlotLeaderRequestConfig, GetSlotLeaderResponse},
    slot_leaders::{GetSlotLeadersRequest, GetSlotLeadersResponse},
    stake_activation::{
        GetStakeActivationRequest,
        GetStakeActivationRequestConfig,
        GetStakeActivationResponse,
    },
    supply::{GetSupplyRequest, GetSupplyRequestConfig, GetSupplyResponse},
    token_account_balance::{
        GetTokenAccountBalanceRequest,
        GetTokenAccountBalanceRequestConfig,
        GetTokenAccountBalanceResponse,
    },
    token_accounts_by_delegate::{
        GetTokenAccountsByDelegateRequest,
        GetTokenAccountsByDelegateRequestConfig,
        GetTokenAccountsByDelegateResponse,
    },
    token_accounts_by_owner::{
        GetTokenAccountsByOwnerRequest,
        GetTokenAccountsByOwnerRequestConfig,
        GetTokenAccountsByOwnerResponse,
    },
    token_largest_accounts::{
        GetTokenLargestAccountsRequest,
        GetTokenLargestAccountsRequestConfig,
        GetTokenLargestAccountsResponse,
    },
    token_supply::{GetTokenSupplyRequest, GetTokenSupplyRequestConfig, GetTokenSupplyResponse},
    transaction::{GetTransactionRequest, GetTransactionRequestConfig, GetTransactionResponse},
    validate_blockhash::{
        ValidateBlockhashRequest,
        ValidateBlockhashRequestConfig,
        ValidateBlockhashResponse,
    },
    version::{GetVersionRequest, GetVersionResponse},
    vote_accounts::{
        GetVoteAccountsRequest,
        GetVoteAccountsRequestConfig,
        GetVoteAccountsResponse,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Encoding {
    JsonParsed,
    Base64,
    Base58,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    slot: u64,
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
