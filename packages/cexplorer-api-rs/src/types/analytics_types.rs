use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::pool_types::{PoolData, PoolInfo, PoolMeta};

pub type WealthCompositionResponse = ResponseCore<Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardforkInfoData {
    pub name: String,
    pub slug: String,
    pub ready: f64,
    #[serde(rename = "inProgress")]
    pub in_progress: f64,
    #[serde(rename = "notStarted")]
    pub not_started: f64,
    #[serde(rename = "releaseDate")]
    #[serde(default)]
    pub release_date: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardforkExchange {
    pub name: String,
    #[serde(rename = "updateOn")]
    pub update_on: String,
    #[serde(rename = "liquidityPercentage")]
    pub liquidity_percentage: f64,
    pub status: String,
    pub logo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardforkDetail {
    pub name: String,
    pub slug: String,
    pub ready: f64,
    #[serde(rename = "inProgress")]
    pub in_progress: f64,
    #[serde(rename = "notStarted")]
    pub not_started: f64,
    #[serde(rename = "releaseDate")]
    #[serde(default)]
    pub release_date: Option<String>,
    pub description: String,
    pub exchanges: Vec<HardforkExchange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolsStat {
    pub count: f64,
    pub version: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardforkPools {
    pub max: String,
    pub stat: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardforkData {
    pub info: HardforkInfoData,
    pub detail: HardforkDetail,
    pub pools: HardforkPools,
}

pub type HardforkResponse = ResponseCore<HardforkData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockVersion {
    pub version: f64,
    pub count: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolBlockVersion {
    pub count: f64,
    pub stake: f64,
    pub version: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxComposition {
    #[serde(default)]
    pub datum: Option<f64>,
    #[serde(default)]
    pub delegation: Option<f64>,
    #[serde(default)]
    pub delegation_vote: Option<f64>,
    #[serde(default)]
    pub drep_registration: Option<f64>,
    #[serde(default)]
    pub gov_action_proposal: Option<f64>,
    #[serde(default)]
    pub ma_tx_mint: Option<f64>,
    #[serde(default)]
    pub ma_tx_out: Option<f64>,
    #[serde(default)]
    pub pool_update: Option<f64>,
    #[serde(default)]
    pub redeemer_data: Option<f64>,
    #[serde(default)]
    pub script: Option<f64>,
    #[serde(default)]
    pub stake_deregistration: Option<f64>,
    #[serde(default)]
    pub stake_registration: Option<f64>,
    #[serde(default)]
    pub tx_metadata: Option<f64>,
    #[serde(default)]
    pub withdrawal: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochAnalyticsStat {
    #[serde(default)]
    pub sum_fee: Option<f64>,
    #[serde(default)]
    pub count_tx: Option<f64>,
    #[serde(default)]
    pub avg_tx_fee: Option<String>,
    #[serde(default)]
    pub count_mint: Option<f64>,
    #[serde(default)]
    pub avg_tx_size: Option<String>,
    #[serde(default)]
    pub count_block: Option<f64>,
    #[serde(default)]
    pub count_datum: Option<f64>,
    #[serde(default)]
    pub count_tx_out: Option<f64>,
    #[serde(default)]
    pub block_version: Option<Vec<BlockVersion>>,
    #[serde(default)]
    pub avg_block_size: Option<String>,
    #[serde(default)]
    pub avg_tx_out_sum: Option<String>,
    #[serde(default)]
    pub count_redeemer: Option<f64>,
    #[serde(default)]
    pub block_producers: Option<f64>,
    #[serde(default)]
    pub count_delegation: Option<f64>,
    #[serde(default)]
    pub count_tx_metadata: Option<f64>,
    #[serde(default)]
    pub avg_tx_script_size: Option<String>,
    #[serde(default)]
    pub count_tx_out_stake: Option<f64>,
    #[serde(default)]
    pub pool_block_version: Option<Vec<PoolBlockVersion>>,
    #[serde(default)]
    pub gov_delegation_vote: Option<f64>,
    #[serde(default)]
    pub max_block_tx_count: Option<f64>,
    #[serde(default)]
    pub count_tx_out_address: Option<f64>,
    #[serde(default)]
    pub count_tx_metadata_with_721: Option<f64>,
    #[serde(default)]
    pub count_tx_out_stake_not_yesterday: Option<f64>,
    #[serde(default)]
    pub count_tx_out_address_not_yesterday: Option<f64>,
    #[serde(default)]
    pub tx_composition: Option<TxComposition>,
    #[serde(default)]
    pub count_pool_relay_uniq: Option<f64>,
    #[serde(default)]
    pub count_pool: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochAnalyticsData {
    pub no: f64,
    #[serde(default)]
    pub stat: Option<EpochAnalyticsStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsRateData {
    pub date: String,
    #[serde(default)]
    pub stat: Option<EpochAnalyticsStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolBlock {
    pub pool_id: String,
    #[serde(default)]
    pub epochs: Option<f64>,
    #[serde(default)]
    pub blocks_minted: Option<f64>,
    #[serde(default)]
    pub blocks_estimated: Option<f64>,
    #[serde(default)]
    pub luck: Option<f64>,
    #[serde(default)]
    pub pool: Option<PoolInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationTx {
    pub slot: f64,
    pub tx_hash: String,
    pub active_epoch_no: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDelegation {
    pub tx: DelegationTx,
    pub pool: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDelegator {
    pub id: String,
    #[serde(default)]
    pub meta: Option<PoolMeta>,
    pub delegation: PoolDelegation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepDelegation {
    pub tx: DelegationTx,
    pub view: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepDelegator {
    pub id: String,
    #[serde(default)]
    pub meta: Option<PoolMeta>,
    pub delegation: DrepDelegation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsTopStakingAccount {
    pub view: String,
    #[serde(default)]
    pub live_stake: Option<f64>,
    #[serde(default)]
    pub script: Option<Value>,
    #[serde(default)]
    pub deleg: Option<PoolDelegator>,
    #[serde(default)]
    pub drep: Option<DrepDelegator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsTopAddress {
    pub address: String,
    pub first: String,
    pub last: String,
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub deleg: Option<PoolDelegator>,
    #[serde(default)]
    pub drep: Option<DrepDelegator>,
}

pub type EpochAnalyticsResponse = ResponseCore<EpochAnalyticsData>;
pub type AnalyticsRateResponse = ResponseCore<Vec<AnalyticsRateData>>;
pub type AnalyticsPoolBlockResponse = ResponseCore<Vec<PoolBlock>>;
pub type AnalyticsTopStakingAccountsResponse = ResponseCore<Vec<AnalyticsTopStakingAccount>>;
pub type AnalyticsTopAddressesResponse = ResponseCore<Vec<AnalyticsTopAddress>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaPot {
    pub epoch_no: f64,
    #[serde(default)]
    pub treasury: Option<f64>,
    #[serde(default)]
    pub reserves: Option<f64>,
    #[serde(default)]
    pub rewards: Option<f64>,
    #[serde(default)]
    pub utxo: Option<f64>,
    #[serde(default)]
    pub deposits_stake: Option<f64>,
    #[serde(default)]
    pub fees: Option<f64>,
    #[serde(default)]
    pub deposits_drep: Option<f64>,
    #[serde(default)]
    pub deposits_proposal: Option<f64>,
}

pub type AnalyticsAdaPotsResponse = ResponseCore<Vec<AdaPot>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupsListDataInfo {
    pub count: f64,
    #[serde(default)]
    pub pool: Option<Value>,
    #[serde(default)]
    pub asset: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupsListData {
    pub name: String,
    pub url: String,
    pub description: String,
    pub param: String,
    pub data: GroupsListDataInfo,
}

pub type GroupsListResponse = ResponseCore<Vec<GroupsListData>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetailItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub ident: String,
    pub info: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetailData {
    pub url: String,
    pub name: String,
    pub description: String,
    pub items: Vec<GroupDetailItem>,
}

pub type GroupDetailResponse = ResponseCore<Vec<GroupDetailData>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AveragePool {
    pub epoch_no: f64,
    #[serde(default)]
    pub avg_delegator: Option<f64>,
    #[serde(default)]
    pub avg_epoch_stake: Option<f64>,
}

pub type AveragePoolResponse = ResponseCore<Vec<AveragePool>>;
