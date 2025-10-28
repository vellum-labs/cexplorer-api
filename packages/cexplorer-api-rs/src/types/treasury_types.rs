use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryDonationStatsEpoch {
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub treasury_donation: Option<f64>,
    #[serde(default)]
    pub rate: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryDonationStats {
    #[serde(default)]
    pub total: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryDonationStatsData {
    pub epoch: Vec<TreasuryDonationStatsEpoch>,
    pub stat: TreasuryDonationStats,
}

pub type TreasuryDonationStatsResponse = ResponseCore<TreasuryDonationStatsData>;
