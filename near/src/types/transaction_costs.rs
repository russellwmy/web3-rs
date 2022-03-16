use super::{ActionCreationConfig, CostConfig, DataReceiptCreationConfig, StorageUsageConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCosts {
    pub action_creation_config: ActionCreationConfig,
    pub action_receipt_creation_config: CostConfig,
    pub burnt_gas_reward: (u64, u64),
    pub data_receipt_creation_config: DataReceiptCreationConfig,
    pub pessimistic_gas_price_inflation_ratio: (u64, u64),
    pub storage_usage_config: StorageUsageConfig,
}
