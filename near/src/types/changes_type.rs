#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangesType {
    SingleAccessKeyChanges,
    AllAccessKeyChanges,
    AccountChanges,
    #[serde(rename = "contract_code_changes")]
    ContractCodeChanges,
    #[serde(rename = "data_changes")]
    ContractStateChanges,
}
