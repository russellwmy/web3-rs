#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestType {
    ViewAccessKey,
    ViewAccessKeyList,
    ViewAccount,
    #[serde(rename = "view_code")]
    ViewContractCode,
    #[serde(rename = "view_state")]
    ViewContractState,
    #[serde(rename = "call_function")]
    CallContractFunction,
}
