use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub ticker: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub extended: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardAccount {
    #[serde(default)]
    pub live_stake: Option<f64>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub epoch_stake: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardSpendableEpoch {
    #[serde(default)]
    pub no: Option<f64>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPool {
    pub id: String,
    pub meta: Meta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardItem {
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(rename = "type")]
    pub reward_type: String,
    #[serde(default)]
    pub earned_epoch: Option<f64>,
    pub account: RewardAccount,
    pub spendable_epoch: RewardSpendableEpoch,
    pub pool: RewardPool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountReward {
    #[serde(default)]
    pub count: Option<f64>,
    pub data: Vec<RewardItem>,
    #[serde(default)]
    #[serde(rename = "prevOffset")]
    pub prev_offset: Option<f64>,
}

pub type AccountRewardResponse = ResponseCore<AccountReward>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDelegationData {
    #[serde(default)]
    pub deposit: Option<f64>,
    #[serde(default)]
    pub epoch_no: Option<f64>,
    pub tx: String,
}

pub type CheckDelegationResponse = ResponseCore<Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalTx {
    pub hash: String,
    #[serde(default)]
    pub out_sum: Option<f64>,
    #[serde(default)]
    pub treasury_donation: Option<f64>,
    #[serde(default)]
    pub size: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalBlock {
    #[serde(default)]
    pub epoch_no: Option<f64>,
    pub hash: String,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalAccount {
    #[serde(default)]
    pub live_stake: Option<f64>,
    #[serde(default)]
    pub script: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalPoolInfo {
    #[serde(default)]
    pub live: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalItem {
    #[serde(default)]
    pub amount: Option<f64>,
    pub tx: WithdrawalTx,
    pub block: WithdrawalBlock,
    pub account: WithdrawalAccount,
    pub pool: WithdrawalPoolInfo,
    pub view: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalsData {
    #[serde(default)]
    pub count: Option<f64>,
    pub data: Vec<WithdrawalItem>,
}

pub type WithdrawalsResponse = ResponseCore<WithdrawalsData>;
