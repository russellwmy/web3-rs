use super::AccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardLayout {
    pub num_shards: u64,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleNightshadeShardLayout {
    pub boundary_accounts: Vec<AccountId>,
    pub fixed_shards: Vec<u64>,
    pub shards_split_map: Vec<Vec<u64>>,
    pub to_parent_shard_map: Vec<u64>,
    pub version: u64,
}
