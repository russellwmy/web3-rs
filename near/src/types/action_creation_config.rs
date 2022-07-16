use super::{AddKeyCost, CostConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionCreationConfig {
    pub add_key_cost: AddKeyCost,
    create_account_cost: CostConfig,
    delete_account_cost: CostConfig,
    delete_key_cost: CostConfig,
    deploy_contract_cost: CostConfig,
    deploy_contract_cost_per_byte: CostConfig,
    function_call_cost: CostConfig,
    function_call_cost_per_byte: CostConfig,
    stake_cost: CostConfig,
    transfer_cost: CostConfig,
}
