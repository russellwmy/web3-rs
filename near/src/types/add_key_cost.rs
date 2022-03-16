use super::CostConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddKeyCost {
    full_access_cost: CostConfig,
    function_call_cost: CostConfig,
    function_call_cost_per_byte: CostConfig,
}
