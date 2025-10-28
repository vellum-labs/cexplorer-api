use serde::{Deserialize, Serialize};
use crate::types::common_types::ResponseCore;
use crate::types::tx_types::{TxBasicInfo, BlockBasicInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInteractionsDataInfo {
    pub purpose: String,
    #[serde(default)]
    pub unit_mem: Option<f64>,
    #[serde(default)]
    pub data_hash: Option<String>,
    #[serde(default)]
    pub data_value: Option<String>,
    #[serde(default)]
    pub unit_steps: Option<f64>,
    pub script_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInteractionsData {
    pub tx: TxBasicInfo,
    pub data: ContractInteractionsDataInfo,
    pub block: BlockBasicInfo,
}

pub type ContractInteractionsResponse = ResponseCore<Vec<ContractInteractionsData>>;
