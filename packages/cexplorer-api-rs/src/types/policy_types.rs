use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::assets_types::AssetPolicy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRoyalties {
    #[serde(default)]
    pub rate: Option<f64>,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStats {
    #[serde(default)]
    pub floor: Option<f64>,
    #[serde(default)]
    pub owners: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    #[serde(default)]
    pub royalties: Option<PolicyRoyalties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDataStats {
    #[serde(default)]
    pub assets: Option<f64>,
    #[serde(default)]
    pub total_count: Option<f64>,
    #[serde(default)]
    pub total_stake: Option<f64>,
    #[serde(default)]
    pub total_address: Option<f64>,
    #[serde(default)]
    pub total_with_data: Option<f64>,
    #[serde(default)]
    pub total_ada_volume: Option<f64>,
    #[serde(default)]
    pub total_asset_volume: Option<f64>,
    #[serde(default)]
    pub total_payment_cred: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyScript {
    #[serde(rename = "type")]
    pub script_type: String,
    pub json: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub mintc: Option<f64>,
    #[serde(default)]
    pub last_mint: Option<String>,
    #[serde(default)]
    pub first_mint: Option<String>,
    #[serde(default)]
    pub stats: Option<PolicyDataStats>,
    #[serde(default)]
    pub script: Option<PolicyScript>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCollection {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub stats: Option<PolicyStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDetail {
    pub id: String,
    pub policy: Policy,
    #[serde(default)]
    pub collection: Option<PolicyCollection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStatData {
    #[serde(default)]
    pub total_count: Option<f64>,
    #[serde(default)]
    pub total_stake: Option<f64>,
    #[serde(default)]
    pub total_address: Option<f64>,
    #[serde(default)]
    pub total_with_data: Option<f64>,
    #[serde(default)]
    pub total_ada_volume: Option<f64>,
    #[serde(default)]
    pub total_asset_volume: Option<f64>,
    #[serde(default)]
    pub total_payment_cred: Option<f64>,
    #[serde(default)]
    pub assets: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStat {
    #[serde(default)]
    pub epoch: Option<f64>,
    #[serde(default)]
    pub stat: Option<PolicyStatData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyOwner {
    pub address: String,
    #[serde(default)]
    pub quantity: Option<f64>,
}

pub type PolicyDetailResponse = ResponseCore<PolicyDetail>;
pub type PolicyStatsResponse = ResponseCore<Vec<PolicyStat>>;
pub type PolicyOwnerResponse = ResponseCore<Vec<PolicyOwner>>;
