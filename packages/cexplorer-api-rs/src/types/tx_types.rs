use serde::{Deserialize, Serialize};
use serde_json::Value;


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
    pub description: String,
    pub url: String,
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