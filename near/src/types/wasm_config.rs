use super::{ExtCosts, LimitConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmConfig {
    ext_costs: ExtCosts,
    grow_mem_cost: u64,
    limit_config: LimitConfig,
    regular_op_cost: u64,
}
