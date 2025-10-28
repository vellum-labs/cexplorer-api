use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::epoch_types::EpochParam;
use crate::types::common_types::ResponseCore;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBasicInfo {
    pub no: Option<u64>,
    pub hash: String,
    pub time: String,
    pub epoch_no: u64,
    pub slot_no: Option<Option<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxBasicInfo {
    pub block: BlockBasicInfo,
    pub fee: Option<u64>,
    pub hash: String,
    pub size: Option<u64>,
    pub deposit: i64,
    pub out_sum: Option<u64>,
    pub script_size: Option<u64>,
    pub invalid_before: Option<Option<u64>>,
    pub invalid_hereafter: Option<u64>,
    pub valid_contract: Value,
    pub treasury_donation: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Withdrawal {
    pub amount: Option<u64>,
    pub stake_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRegistry {
    pub ticker: String,
    pub name: String,
    pub decimals: Option<u64>,
    pub has_logo: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxAsset {
    pub name: String,
    pub quantity: i64,
    pub registry: AssetRegistry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatumValue {
    pub fields: Vec<Value>,
    pub constructor: Option<Option<u64>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineDatum {
    pub bytes: String,
    pub value: DatumValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceScript {
    pub hash: String,
    pub size: Option<u64>,
    #[serde(rename = "type")]
    pub script_type: String,
    pub bytes: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInfo {
    pub value: Option<u64>,
    pub tx_id: Option<u64>,
    pub tx_hash: String,
    pub tx_index: Option<u64>,
    pub asset: Option<Vec<TxAsset>>,
    pub datum_hash: Option<String>,
    pub stake_addr: String,
    pub inline_datum: Option<InlineDatum>,
    pub reference_script: Option<ReferenceScript>,
    pub payment_addr_cred: String,
    pub payment_addr_bech32: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mint {
    pub quantity: i64,
    pub name: String,
    pub registry: AssetRegistry
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxDetailParams {
    block_no: Option<i64>,
    slot_no: Option<i64>,
    hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxDetailData {
    pub block: BlockBasicInfo,
    pub fee: Option<u64>,
    pub hash: String,
    pub size: Option<u64>,
    pub deposit: i64,
    pub out_sum: Option<u64>,
    pub script_size: Option<u64>,
    pub invalid_before: Option<Option<u64>>,
    pub invalid_hereafter: Option<u64>,
    pub valid_contract: Value,
    pub treasury_donation: Option<u64>,
    pub epoch_param: EpochParam,
    pub all_inputs: Option<Vec<TxInfo>>,
    pub all_outputs: Option<Vec<TxInfo>>,
    pub all_collateral_outputs: Option<Vec<TxInfo>>,
    pub all_withdrawals: Option<Vec<Withdrawal>>,
    pub collateral_inputs: Option<Vec<TxInfo>>,
    pub reference_inputs: Option<Vec<TxInfo>>,
    pub metadata: Option<Value>,
    pub mints: Option<Vec<Mint>>,
    pub defi: Option<Vec<Value>>,
    pub plutus_contracts: Option<Vec<Value>>,
    pub pool: Option<Value>,
    pub rate: Option<Value>,
    pub delegation: Option<Vec<Value>>,
    pub governance: Option<Governance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Governance {
    pub voting_procedure: Vec<VotingProcedure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingProcedure {
    pub info: VotingInfo,
    pub vote: String,
    pub proposal: Value,
    pub voter_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingInfo {
    pub id: String,
    pub meta: Value,
    pub power: Power,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Power {
    pub stake: f64,
    pub represented_by: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxDetailResponse {
    #[serde(default)]
    pub code: Option<f64>,
    #[serde(default)]
    pub tokens: Option<f64>,
    pub data: TxDetailData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxListData {
    #[serde(default)]
    pub count: Option<f64>,
    pub data: Vec<TxBasicInfo>,
}

pub type TxListResponse = ResponseCore<TxListData>;
