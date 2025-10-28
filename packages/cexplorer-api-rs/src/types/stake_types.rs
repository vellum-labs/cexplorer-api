use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::address_types::AddressAsset;
use crate::types::user_types::User;
use crate::types::tx_types::BlockBasicInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeDelegation {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub meta: Option<String>,
    pub delegation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeReward {
    #[serde(default)]
    pub total: Option<f64>,
    #[serde(default)]
    pub withdrawn: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeInfo {
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub slot_update: Option<f64>,
    #[serde(default)]
    pub slot_first_registered: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeLive {
    pub deleg: StakeDelegation,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub accounts: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeActive {
    pub deleg: StakeDelegation,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub reward: Option<f64>,
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub epoch_delay: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeData {
    pub info: StakeInfo,
    pub live: StakeLive,
    pub active: StakeActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeDetailData {
    pub view: String,
    #[serde(default)]
    pub asset: Option<Vec<AddressAsset>>,
    pub hash_raw: String,
    #[serde(default)]
    pub script_hash: Option<String>,
    pub reward: StakeReward,
    pub stake: StakeData,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub adahandle: Option<Value>,
    #[serde(default)]
    pub vote: Option<Value>,
}

pub type StakeDetailResponse = ResponseCore<StakeDetailData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRegistrationData {
    pub hash_raw: String,
    pub view: String,
    #[serde(default)]
    pub script_hash: Option<String>,
    #[serde(default)]
    pub epoch_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRegistrationsDataItem {
    pub tx: Value,
    pub data: StakeRegistrationData,
    pub block: BlockBasicInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeRegistrationsData {
    #[serde(default)]
    pub count: Option<f64>,
    pub data: Vec<StakeRegistrationsDataItem>,
}

pub type StakeRegistrationsResponse = ResponseCore<StakeRegistrationsData>;
