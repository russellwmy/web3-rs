#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreationConfig {
    min_allowed_top_level_account_length: u64,
    registrar_account_id: String,
}
