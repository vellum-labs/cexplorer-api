use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochParam {
    pub nonce: String,
    pub epoch_no: Option<u64>,
    pub influence: Option<f64>,
    pub max_epoch: Option<u64>,
    pub min_fee_a: Option<u64>,
    pub min_fee_b: Option<u64>,
    pub price_mem: Option<f64>,
    pub price_step: Option<f64>,
    pub key_deposit: Option<u64>,
    pub max_bh_size: Option<u64>,
    pub max_tx_size: Option<u64>,
    pub max_val_size: Option<u64>,
    pub pool_deposit: Option<u64>,
    #[serde(default)]
    pub drep_activity: Option<u64>,
    pub extra_entropy: Option<String>,
    pub max_tx_ex_mem: Option<u64>,
    pub min_pool_cost: Option<u64>,
    pub max_block_size: Option<u64>,
    pub min_utxo_value: Option<u64>,
    pub protocol_major: Option<u64>,
    pub protocol_minor: Option<u64>,
    pub max_tx_ex_steps: Option<u64>,
    pub decentralisation: Option<f64>,
    pub max_block_ex_mem: Option<u64>,
    #[serde(default)]
    pub dvt_p_p_gov_group: Option<Option<f64>>,
    pub collateral_percent: Option<f64>,
    #[serde(default)]
    pub committee_min_size: Option<u64>,
    pub max_block_ex_steps: Option<u64>,
    pub optimal_pool_count: Option<u64>,
    pub coins_per_utxo_size: Option<u64>,
    #[serde(default)]
    pub gov_action_lifetime: Option<u64>,
    #[serde(default)]
    pub dvt_committee_normal: Option<Option<f64>>,
    pub monetary_expand_rate: Option<f64>,
    #[serde(default)]
    pub pvt_committee_normal: Option<Option<f64>>,
    #[serde(default)]
    pub pvtpp_security_group: Option<Option<f64>>,
    pub treasury_growth_rate: Option<f64>,
    #[serde(default)]
    pub dvt_p_p_network_group: Option<Option<f64>>,
    pub max_collateral_inputs: Option<u64>,
    #[serde(default)]
    pub dvt_p_p_economic_group: Option<Option<f64>>,
    #[serde(default)]
    pub dvt_p_p_technical_group: Option<Option<f64>>,
    #[serde(default)]
    pub dvt_treasury_withdrawal: Option<Option<f64>>,
    #[serde(default)]
    pub dvt_hard_fork_initiation: Option<Option<f64>>,
    #[serde(default)]
    pub dvt_motion_no_confidence: Option<Option<f64>>,
    #[serde(default)]
    pub pvt_hard_fork_initiation: Option<Option<f64>>,
    #[serde(default)]
    pub pvt_motion_no_confidence: Option<Option<f64>>,
    #[serde(default)]
    pub committee_max_term_length: Option<u64>,
    #[serde(default)]
    pub dvt_update_to_constitution: Option<Option<f64>>,
    #[serde(default)]
    pub dvt_committee_no_confidence: Option<Option<f64>>,
    #[serde(default)]
    pub pvt_committee_no_confidence: Option<Option<f64>>,
    #[serde(default)]
    pub min_fee_ref_script_cost_per_byte: Option<Option<f64>>,
}

// Response types for epoch endpoints
use crate::types::common_types::ResponseCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochListData {
    pub no: Option<u64>,
    pub start_time: String,
    pub end_time: String,
    pub blk_count: Option<u64>,
    pub tx_count: Option<u64>,
    pub out_sum: Option<u64>,
    pub fees: Option<u64>,
    #[serde(default)]
    pub params: Option<Vec<EpochParam>>,
    #[serde(default)]
    pub params_active: Option<Vec<EpochParam>>,
    #[serde(default)]
    pub stats: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochList {
    pub count: Option<u64>,
    pub data: Vec<EpochListData>,
}

pub type EpochListResponse = ResponseCore<EpochList>;
pub type EpochDetailParamResponse = ResponseCore<EpochParam>;
pub type EpochDetailStatsResponse = ResponseCore<serde_json::Value>;