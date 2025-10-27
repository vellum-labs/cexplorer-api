use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochParam {
    pub nonce: String,
    pub epoch_no: u64,
    pub influence: f64,
    pub max_epoch: u64,
    pub min_fee_a: u64,
    pub min_fee_b: u64,
    pub price_mem: f64,
    pub price_step: f64,
    pub key_deposit: u64,
    pub max_bh_size: u64,
    pub max_tx_size: u64,
    pub max_val_size: u64,
    pub pool_deposit: u64,
    #[serde(default)]
    pub drep_activity: Option<u64>,
    pub extra_entropy: Option<String>,
    pub max_tx_ex_mem: u64,
    pub min_pool_cost: u64,
    pub max_block_size: u64,
    pub min_utxo_value: u64,
    pub protocol_major: u64,
    pub protocol_minor: u64,
    pub max_tx_ex_steps: u64,
    pub decentralisation: f64,
    pub max_block_ex_mem: u64,
    #[serde(default)]
    pub dvt_p_p_gov_group: Option<f64>,
    pub collateral_percent: f64,
    #[serde(default)]
    pub committee_min_size: Option<u64>,
    pub max_block_ex_steps: u64,
    pub optimal_pool_count: u64,
    pub coins_per_utxo_size: u64,
    #[serde(default)]
    pub gov_action_lifetime: Option<u64>,
    #[serde(default)]
    pub dvt_committee_normal: Option<f64>,
    pub monetary_expand_rate: f64,
    #[serde(default)]
    pub pvt_committee_normal: Option<f64>,
    #[serde(default)]
    pub pvtpp_security_group: Option<f64>,
    pub treasury_growth_rate: f64,
    #[serde(default)]
    pub dvt_p_p_network_group: Option<f64>,
    pub max_collateral_inputs: u64,
    #[serde(default)]
    pub dvt_p_p_economic_group: Option<f64>,
    #[serde(default)]
    pub dvt_p_p_technical_group: Option<f64>,
    #[serde(default)]
    pub dvt_treasury_withdrawal: Option<f64>,
    #[serde(default)]
    pub dvt_hard_fork_initiation: Option<f64>,
    #[serde(default)]
    pub dvt_motion_no_confidence: Option<f64>,
    #[serde(default)]
    pub pvt_hard_fork_initiation: Option<f64>,
    #[serde(default)]
    pub pvt_motion_no_confidence: Option<f64>,
    #[serde(default)]
    pub committee_max_term_length: Option<u64>,
    #[serde(default)]
    pub dvt_update_to_constitution: Option<f64>,
    #[serde(default)]
    pub dvt_committee_no_confidence: Option<f64>,
    #[serde(default)]
    pub pvt_committee_no_confidence: Option<f64>,
    #[serde(default)]
    pub min_fee_ref_script_cost_per_byte: Option<f64>,
}