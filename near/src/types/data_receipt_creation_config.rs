use super::CostConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataReceiptCreationConfig {
    pub base_cost: CostConfig,
    pub cost_per_byte: CostConfig,
}
