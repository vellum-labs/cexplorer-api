use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenListStat {
    #[serde(rename = "1d")]
    #[serde(default)]
    pub one_day: Option<f64>,
    #[serde(rename = "1m")]
    #[serde(default)]
    pub one_month: Option<f64>,
    #[serde(rename = "1w")]
    #[serde(default)]
    pub one_week: Option<f64>,
    #[serde(rename = "2w")]
    #[serde(default)]
    pub two_weeks: Option<f64>,
    #[serde(rename = "3m")]
    #[serde(default)]
    pub three_months: Option<f64>,
    #[serde(default)]
    pub rows: Option<Value>,
    #[serde(default)]
    pub today: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenListItem {
    pub assetname: String,
    #[serde(default)]
    pub registry: Option<Value>,
    #[serde(default)]
    pub stat: Option<DeFiTokenListStat>,
    #[serde(default)]
    pub is_verified: Option<bool>,
    #[serde(default)]
    pub price_ada: Option<f64>,
    pub updated: String,
    #[serde(default)]
    pub liquidity_ada: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenListRecent24h {
    #[serde(default)]
    pub user: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenListData {
    #[serde(default)]
    pub count: Option<f64>,
    pub data: Vec<DeFiTokenListItem>,
    #[serde(default)]
    pub summary: Option<Value>,
    pub recent_24h: DeFiTokenListRecent24h,
}

pub type DeFiTokenListResponse = ResponseCore<DeFiTokenListData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenStatDaily {
    pub date: String,
    #[serde(default)]
    pub tokens: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub trade: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenStatDataItem {
    pub update_date: String,
    #[serde(default)]
    pub details: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenStat {
    pub daily: Vec<DeFiTokenStatDaily>,
    pub data: Vec<DeFiTokenStatDataItem>,
}

pub type DeFiTokenStatResponse = ResponseCore<DeFiTokenStat>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiTokenInOut {
    pub name: String,
    #[serde(default)]
    pub registry: Option<Value>,
    #[serde(default)]
    pub stat: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiUser {
    pub account: String,
    pub address: String,
    #[serde(default)]
    pub balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiOrderBlock {
    #[serde(default)]
    pub no: Option<f64>,
    pub hash: String,
    pub time: String,
    #[serde(default)]
    pub epoch_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiOrder {
    pub token_in: DeFiTokenInOut,
    pub token_out: DeFiTokenInOut,
    pub user: DeFiUser,
    pub dex: String,
    pub status: String,
    #[serde(default)]
    pub amount_in: Option<f64>,
    #[serde(default)]
    pub expected_out_amount: Option<f64>,
    #[serde(default)]
    pub actual_out_amount: Option<f64>,
    pub submission_time: String,
    pub last_update: String,
    pub tx_hash: String,
    pub update_tx_hash: String,
    #[serde(default)]
    pub is_stop_loss: Option<bool>,
    #[serde(default)]
    pub is_oor: Option<bool>,
    #[serde(default)]
    pub is_dexhunter: Option<bool>,
    #[serde(default)]
    pub batcher_fee: Option<f64>,
    #[serde(default)]
    pub deposit: Option<f64>,
    pub block: DeFiOrderBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiOrderListData {
    pub data: Vec<DeFiOrder>,
    #[serde(default)]
    pub count: Option<f64>,
}

pub type DeFiOrderListResponse = ResponseCore<DeFiOrderListData>;
