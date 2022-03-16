mod access_key;
mod account_creation_config;
mod account_key;
mod action;
mod action_creation_config;
mod add_key_cost;
mod block_details;
mod block_id;
mod changes;
mod changes_cause;
mod changes_type;
mod chunk;
mod chunk_details;
mod cost_config;
mod data_receipt_creation_config;
mod ext_costs;
mod finality;
mod genesis_config;
mod kickout;
mod limit_config;
mod network_info;
mod node_status;
mod node_version;
mod outcome;
mod peer;
mod permission;
mod producer;
mod proposal;
mod protocol_config;
mod public_key;
mod public_key_ed25519;
mod public_key_secp256k1;
mod receipt;
mod request_type;
mod runtime_config;
mod shard_layout;
mod storage_usage_config;
mod sync_info;
mod transaction;
mod transaction_costs;
mod validation_status;
mod validator;
mod wasm_config;

use serde::Serialize;
pub use {
    access_key::*,
    account_creation_config::*,
    account_key::*,
    action::*,
    action_creation_config::*,
    add_key_cost::*,
    block_details::*,
    block_id::*,
    changes::*,
    changes_cause::*,
    changes_type::*,
    chunk::*,
    chunk_details::*,
    cost_config::*,
    data_receipt_creation_config::*,
    ext_costs::*,
    finality::*,
    genesis_config::*,
    kickout::*,
    limit_config::*,
    network_info::*,
    node_status::*,
    node_version::*,
    outcome::*,
    peer::*,
    permission::*,
    producer::*,
    proposal::*,
    protocol_config::*,
    public_key::*,
    public_key_ed25519::*,
    public_key_secp256k1::*,
    receipt::*,
    request_type::*,
    runtime_config::*,
    shard_layout::*,
    storage_usage_config::*,
    sync_info::*,
    transaction::*,
    transaction_costs::*,
    validation_status::*,
    validator::*,
    wasm_config::*,
};

pub type BlockHeight = u64;
pub type BlockHash = String;
pub type AccountId = String;
pub type EpochId = String;
pub type GasPrice = String;
pub type Balance = String;

#[derive(Debug, Clone, Deserialize)]
pub enum FinalityOrHash {
    Finality(Finality),
    Hash(String),
}

impl Serialize for FinalityOrHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FinalityOrHash::Finality(finality) => finality.serialize(serializer),
            FinalityOrHash::Hash(hash) => hash.serialize(serializer),
        }
    }
}
