#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostConfig {
    pub execution: u64,
    pub send_not_sir: u64,
    pub send_sir: u64,
}
