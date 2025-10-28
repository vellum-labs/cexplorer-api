use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::block_types::Rate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscBasicBlock {
    pub hash: String,
    pub time: String,
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub block_no: Option<f64>,
    #[serde(default)]
    pub slot_no: Option<f64>,
    #[serde(default)]
    pub proto: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscBasicAd {
    pub data: Value,
    #[serde(rename = "type")]
    pub ad_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscBasicVersion {
    #[serde(rename = "const")]
    #[serde(default)]
    pub const_version: Option<f64>,
    #[serde(default)]
    pub rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscBasicInstance {
    #[serde(default)]
    pub readonly: Option<bool>,
    pub server: String,
    pub snapshot: String,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscBasicData {
    pub block: MiscBasicBlock,
    #[serde(default)]
    pub ads: Option<Vec<MiscBasicAd>>,
    pub version: MiscBasicVersion,
    pub instance: MiscBasicInstance,
    pub rate: Rate,
    pub rate_day: Rate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscBasicResponse {
    #[serde(default)]
    pub code: Option<f64>,
    pub data: MiscBasicData,
    #[serde(default)]
    pub tokens: Option<f64>,
    #[serde(default)]
    pub ex: Option<f64>,
    #[serde(default)]
    pub debug: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicRate {
    pub date: String,
    #[serde(default)]
    pub adausd: Option<f64>,
    #[serde(default)]
    pub btcusd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscRateData {
    pub rates: Vec<BasicRate>,
}

pub type MiscRateResponse = ResponseCore<MiscRateData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscConstLabel {
    pub name: String,
    #[serde(rename = "type")]
    pub label_type: String,
    pub label: String,
    #[serde(default)]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscConstData {
    #[serde(default)]
    pub no: Option<f64>,
    #[serde(default)]
    pub epoch: Option<Value>,
    #[serde(default)]
    pub circulating_supply: Option<f64>,
    #[serde(default)]
    pub labels: Option<Vec<MiscConstLabel>>,
    #[serde(default)]
    pub epoch_param: Option<Value>,
    #[serde(default)]
    pub epoch_stat: Option<Value>,
    #[serde(default)]
    pub live_stake: Option<f64>,
}

pub type MiscConstResponse = ResponseCore<MiscConstData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscMarketStatus {
    #[serde(default)]
    pub low: Option<f64>,
    #[serde(default)]
    pub high: Option<f64>,
    #[serde(default)]
    pub open: Option<f64>,
    #[serde(default)]
    pub close: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    pub time_open: String,
    #[serde(default)]
    pub market_cap: Option<f64>,
    pub time_close: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscMarketData {
    pub date: String,
    #[serde(default)]
    pub epoch_no: Option<f64>,
    pub need_fix: String,
    pub ada: MiscMarketStatus,
    pub btc: MiscMarketStatus,
    pub fiat: Value,
}

pub type MiscMarketResponse = ResponseCore<MiscMarketData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscSearchExtra {
    pub icon: String,
    #[serde(rename = "type")]
    pub extra_type: String,
    pub value: Value,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscSearch {
    pub url: String,
    pub ident: String,
    pub title: String,
    pub category: String,
    pub extra: MiscSearchExtra,
}

pub type MiscSearchResponse = ResponseCore<Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub applied: Option<String>,
    pub date_start: String,
    pub date_end: String,
    pub description: String,
    pub options: Vec<String>,
    pub state: String,
    #[serde(default)]
    pub vote: Option<Value>,
    #[serde(default)]
    pub result: Option<Value>,
}

pub type PollListResponse = ResponseCore<Vec<Poll>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscValidate {
    #[serde(rename = "type")]
    pub validate_type: String,
    pub ident: String,
    #[serde(default)]
    pub valid: Option<bool>,
}

pub type MiscValidateResponse = ResponseCore<MiscValidate>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscApiTier {
    #[serde(default)]
    pub rq_min: Option<f64>,
    #[serde(default)]
    pub rq_day: Option<f64>,
    #[serde(default)]
    pub tok_day: Option<f64>,
    pub license: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscApiData {
    pub starter: MiscApiTier,
    pub basic: MiscApiTier,
    pub pro: MiscApiTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscApiPlans {
    pub plans: MiscApiData,
}

pub type MiscApiResponse = ResponseCore<MiscApiPlans>;
