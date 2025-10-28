use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::pool_types::PoolInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    #[serde(default)]
    pub amount: Option<f64>,
    pub pool_id: String,
    #[serde(default)]
    pub earned_epoch: Option<f64>,
    #[serde(default)]
    pub spendable_epoch: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveStake {
    #[serde(default)]
    pub epoch: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stake {
    #[serde(default)]
    pub live: Option<f64>,
    #[serde(default)]
    pub active: Option<Vec<ActiveStake>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    #[serde(default)]
    pub slot_no: Option<f64>,
    #[serde(default)]
    pub pool: Option<PoolInfo>,
    pub tx_hash: String,
    #[serde(default)]
    pub active_epoch_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationPair {
    pub live: Delegation,
    pub active: Delegation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationStateData {
    pub view: String,
    #[serde(default)]
    pub stake: Option<Stake>,
    #[serde(default)]
    pub reward: Option<Vec<Reward>>,
    #[serde(default)]
    pub script: Option<Value>,
    pub hash_raw: String,
    pub delegation: DelegationPair,
}

pub type DelegationStateResponse = ResponseCore<Vec<DelegationStateData>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationTx {
    pub hash: String,
    #[serde(default)]
    pub slot_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationPoolPair {
    pub live: PoolInfo,
    pub previous: PoolInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationData {
    pub tx: DelegationTx,
    pub view: String,
    pub pool: DelegationPoolPair,
    #[serde(default)]
    pub active_stake: Option<f64>,
    #[serde(default)]
    pub live_stake: Option<f64>,
    #[serde(default)]
    pub active_epoch_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationAccount {
    #[serde(default)]
    pub script: Option<Value>,
    #[serde(default)]
    pub live_stake: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationToRetiredData {
    pub pool: PoolInfo,
    pub view: String,
    pub account: DelegationAccount,
}

pub type DelegationResponse = ResponseCore<Vec<DelegationData>>;
pub type DelegationToRetiredResponse = ResponseCore<Vec<DelegationToRetiredData>>;
