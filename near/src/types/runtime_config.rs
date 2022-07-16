use super::{AccountCreationConfig, TransactionCosts, WasmConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub account_creation_config: AccountCreationConfig,
    pub storage_amount_per_byte: String,
    pub transaction_costs: TransactionCosts,
    pub wasm_config: WasmConfig,
}
