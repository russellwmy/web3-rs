#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsageConfig {
    pub num_bytes_account: u64,
    pub num_extra_bytes_record: u64,
}
