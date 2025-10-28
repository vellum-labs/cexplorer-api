use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::assets_types::MetadataTx;
use crate::types::tx_types::Label;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatRedeemer {
    #[serde(default)]
    pub sum: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub redeemers: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatOut {
    #[serde(default)]
    pub sum: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub address: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatTxMint {
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub assets: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatTxPaymentCred {
    #[serde(default)]
    pub out: Option<ScriptStatOut>,
    #[serde(default)]
    pub tx_mint: Option<ScriptStatTxMint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatItemData {
    #[serde(default)]
    pub redeemer: Option<ScriptStatRedeemer>,
    #[serde(default)]
    pub tx_payment_cred: Option<ScriptStatTxPaymentCred>,
    #[serde(default)]
    pub tx_reference_script: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatItemInfo {
    #[serde(default)]
    pub data: Option<ScriptStatItemData>,
    #[serde(default)]
    pub epoch_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatItem {
    #[serde(default)]
    pub item: Option<ScriptStatItemInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStatTotal {
    #[serde(default)]
    pub epochs: Option<f64>,
    #[serde(default)]
    pub interactions: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptPurpose {
    #[serde(default)]
    pub count: Option<f64>,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDetailData {
    pub tx: MetadataTx,
    pub hash: String,
    #[serde(default)]
    pub json: Option<Value>,
    #[serde(rename = "type")]
    pub script_type: String,
    #[serde(default)]
    pub label: Option<Label>,
    pub bytecode: String,
    #[serde(default)]
    pub serialised_size: Option<f64>,
    #[serde(default)]
    pub purpose: Option<Vec<ScriptPurpose>>,
    #[serde(default)]
    pub stat: Option<Vec<ScriptStatItem>>,
    #[serde(default)]
    pub stat_total: Option<ScriptStatTotal>,
}

pub type ScriptDetailResponse = ResponseCore<ScriptDetailData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDetailRedeemerTx {
    pub hash: String,
    #[serde(default)]
    pub slot_no: Option<f64>,
    #[serde(default)]
    pub out_sum: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDetailRedeemerData {
    #[serde(default)]
    pub bytes: Option<String>,
    #[serde(default)]
    pub int: Option<f64>,
    #[serde(default)]
    pub constructor: Option<f64>,
    #[serde(default)]
    pub fields: Option<Vec<Value>>,
    #[serde(default)]
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochParam {
    #[serde(default)]
    pub max_tx_ex_mem: Option<f64>,
    #[serde(default)]
    pub max_tx_ex_steps: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptDetailRedeemerDataItem {
    pub tx: ScriptDetailRedeemerTx,
    #[serde(default)]
    pub fee: Option<f64>,
    pub data: ScriptDetailRedeemerData,
    pub purpose: String,
    #[serde(default)]
    pub unit_mem: Option<f64>,
    #[serde(default)]
    pub unit_steps: Option<f64>,
    #[serde(default)]
    pub epoch_param: Option<EpochParam>,
}

pub type ScriptDetailRedeemerResponse = ResponseCore<Vec<ScriptDetailRedeemerDataItem>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListStatItem {
    #[serde(default)]
    pub redeemer: Option<ScriptStatRedeemer>,
    #[serde(default)]
    pub tx_payment_cred: Option<ScriptStatTxPaymentCred>,
    #[serde(default)]
    pub tx_reference_script: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptStat {
    #[serde(default)]
    pub recent: Option<ScriptListStatItem>,
    #[serde(default)]
    pub previous: Option<ScriptListStatItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptListData {
    pub hash: String,
    #[serde(rename = "type")]
    pub script_type: String,
    #[serde(default)]
    pub serialised_size: Option<f64>,
    #[serde(default)]
    pub is_live: Option<bool>,
    #[serde(default)]
    pub stat: Option<ScriptStat>,
    #[serde(default)]
    pub label: Option<Label>,
}

pub type ScriptListResponse = ResponseCore<Vec<ScriptListData>>;
