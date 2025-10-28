use serde::{Deserialize, Serialize};
use serde_json::Value;

fn deserialize_null_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetaExtended {
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub github_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub reddit_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub twitch_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub discord_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub twitter_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub youtube_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub facebook_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub telegram_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub instagram_handle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMeta {
    pub ticker: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub extended: Option<PoolMetaExtended>,
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub id: String,
    pub meta: Option<PoolMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolEpochBlock {
    #[serde(default)]
    pub minted: Option<f64>,
    #[serde(default)]
    pub estimated: Option<f64>,
    #[serde(default)]
    pub luck: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolEpochReward {
    #[serde(default)]
    pub leader_lovelace: Option<f64>,
    #[serde(default)]
    pub leader_pct: Option<f64>,
    #[serde(default)]
    pub member_lovelace: Option<f64>,
    #[serde(default)]
    pub member_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolEpochData {
    #[serde(default)]
    pub epoch_stake: Option<f64>,
    #[serde(default)]
    pub delegators: Option<f64>,
    #[serde(default)]
    pub block: Option<PoolEpochBlock>,
    #[serde(default)]
    pub reward: Option<PoolEpochReward>,
    #[serde(default)]
    pub pledged: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolEpoch {
    pub no: u64,
    pub data: PoolEpochData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolTxBasic {
    pub hash: String,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolOwner {
    pub view: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolTxWithId {
    pub hash: String,
    pub time: String,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolUpdateRetireItem {
    pub active_epoch_no: u64,
    pub fixed_cost: u64,
    pub index: u64,
    pub margin: f64,
    #[serde(default)]
    pub meta_id: Option<u64>,
    pub pledge: u64,
    pub reward_addr: String,
    pub tx: PoolTxWithId,
    pub owner: Vec<PoolOwner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolRetireItem {
    #[serde(default)]
    pub tx_id: Option<u64>,
    #[serde(default)]
    pub index: Option<u64>,
    #[serde(default)]
    pub retiring_epoch: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolRetire {
    pub live: PoolRetireItem,
    pub active: PoolRetireItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolUpdateRetire {
    pub active: PoolUpdateRetireItem,
    pub live: PoolUpdateRetireItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStatsItem {
    #[serde(default)]
    pub epochs: Option<f64>,
    #[serde(default)]
    pub luck: Option<f64>,
    #[serde(default)]
    pub roa: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    #[serde(default)]
    pub lifetime: Option<PoolStatsItem>,
    #[serde(default)]
    pub recent: Option<PoolStatsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolBlocks {
    #[serde(default)]
    pub epoch: Option<f64>,
    #[serde(default)]
    pub total: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolData {
    pub pool_id: String,
    pub pool_id_hash_raw: String,
    #[serde(default)]
    pub active_stake: Option<f64>,
    #[serde(default)]
    pub live_stake: Option<f64>,
    #[serde(default)]
    pub epochs: Option<Value>,
    #[serde(default)]
    pub active_epochs: Option<f64>,
    #[serde(default)]
    pub pool_name: Option<PoolMeta>,
    #[serde(default)]
    pub pool_retire: Option<PoolRetire>,
    #[serde(default)]
    pub pool_update: Option<PoolUpdateRetire>,
    #[serde(default)]
    pub stats: Option<PoolStats>,
    #[serde(default)]
    pub pledged: Option<f64>,
    #[serde(default)]
    pub blocks: Option<PoolBlocks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolsList {
    pub count: u64,
    pub data: Vec<PoolData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDetailData {
    pub pool_id: String,
    pub hash_raw: String,
    #[serde(default)]
    pub active_stake: Option<f64>,
    #[serde(default)]
    pub live_stake: Option<f64>,
    #[serde(default)]
    pub epochs: Option<Vec<PoolEpoch>>,
    #[serde(default)]
    pub active_epochs: Option<f64>,
    #[serde(default)]
    pub delegators: Option<f64>,
    #[serde(default)]
    pub registered: Option<String>,
    #[serde(default)]
    pub pool_name: Option<PoolMeta>,
    #[serde(default)]
    pub pool_retire: Option<PoolRetire>,
    #[serde(default)]
    pub pool_update: Option<PoolUpdateRetire>,
    #[serde(default)]
    pub stats: Option<PoolStats>,
    #[serde(default)]
    pub pledged: Option<f64>,
    #[serde(default)]
    pub blocks: Option<PoolBlocks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolEpochCert {
    pub tx: String,
    pub margin: f64,
    pub pledge: u64,
    pub fixed_cost: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolRewardData {
    pub no: u64,
    #[serde(default)]
    pub active_stake: Option<u64>,
    #[serde(default)]
    pub block: Option<PoolEpochBlock>,
    #[serde(default)]
    pub reward: Option<PoolEpochReward>,
    #[serde(default)]
    pub delegator: Option<u64>,
    #[serde(default)]
    pub epoch_stake: Option<u64>,
    #[serde(default)]
    pub pledged: Option<u64>,
    #[serde(default)]
    pub cert: Option<PoolEpochCert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolRewards {
    pub count: u64,
    pub data: Vec<PoolRewardData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolBlocksData {
    pub date: String,
    pub block: PoolBlocksStat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolBlocksStat {
    pub count: u64,
    pub avg_tx_count: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDelegationTx {
    pub slot: u64,
    pub tx_hash: String,
    pub active_epoch_no: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfoWithDelegation {
    pub id: String,
    #[serde(default)]
    pub meta: Option<PoolMeta>,
    #[serde(default)]
    pub tx: Option<PoolDelegationTx>,
    pub pool: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDelegatorData {
    #[serde(default)]
    pub view: Option<String>,
    pub script: String,
    #[serde(default)]
    pub slot_update: Option<u64>,
    #[serde(default)]
    pub slot_first_registered: Option<u64>,
    #[serde(default)]
    pub live_stake: Option<u64>,
    pub live_pool: PoolInfoWithDelegation,
    pub active_pool: PoolInfoWithDelegation,
    pub previous_pool: PoolInfoWithDelegation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDelegators {
    pub count: u64,
    pub data: Vec<PoolDelegatorData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAccountItem {
    pub view: String,
    pub active_stake: u64,
    pub live_stake: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAccount {
    pub reward: PoolAccountItem,
    pub owner: PoolAccountItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolUpdateMeta {
    pub hash: String,
    pub url: String,
    pub data: PoolMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolUpdateData {
    pub tx_hash: String,
    pub time: String,
    pub active_epoch_no: u64,
    pub vrf_key_hash: String,
    pub pledge: u64,
    pub margin: String,
    pub fixed_cost: u64,
    pub account: PoolAccount,
    pub meta: PoolUpdateMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolUpdates {
    pub count: u64,
    pub data: Vec<PoolUpdateData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAwardDetail {
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAward {
    pub time: String,
    pub category: String,
    #[serde(rename = "type")]
    pub award_type: String,
    pub detail: PoolAwardDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAwards {
    pub count: u64,
    pub data: Vec<PoolAward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDelegatorStats {
    pub count: u64,
    pub data: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolRelay {
    #[serde(default)]
    pub ipv4: Option<String>,
    #[serde(default)]
    pub ipv6: Option<String>,
    pub dns_name: String,
    #[serde(default)]
    pub dns_srv_name: Option<String>,
    pub port: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAboutPool {
    pub id: String,
    pub meta: PoolMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAboutData {
    pub pool: PoolAboutPool,
    pub relay: Vec<PoolRelay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMarginsPoolName {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub ticker: Option<String>,
    #[serde(default)]
    pub extended: Option<PoolMetaExtended>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMarginsPoolRetire {
    #[serde(default)]
    pub live: Option<Value>,
    #[serde(default)]
    pub active: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMarginsPoolUpdate {
    pub live: Value,
    pub active: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMarginsStats {
    pub recent: PoolStatsItem,
    pub lifetime: PoolStatsItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopDelegator {
    pub view: String,
    pub stake: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastBlock {
    pub proto: u64,
    pub slot_no: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMarginsData {
    pub active_epochs: u64,
    #[serde(default)]
    pub active_stake: Option<u64>,
    pub blocks: PoolBlocks,
    pub delegators: u64,
    pub epochs: Value,
    pub last_block: LastBlock,
    #[serde(default)]
    pub live_stake: Option<u64>,
    pub pledged: u64,
    pub pool_id: String,
    pub pool_id_hash_raw: String,
    pub pool_name: TopMarginsPoolName,
    pub pool_retire: TopMarginsPoolRetire,
    pub pool_update: TopMarginsPoolUpdate,
    pub stats: TopMarginsStats,
    pub top_delegator: TopDelegator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMarginsWithDelegators {
    pub count: u64,
    pub data: Vec<TopMarginsData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetiredPoolStat {
    pub count: u64,
    pub stake: u64,
    pub accounts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetiredPoolItemStat {
    pub live: u64,
    #[serde(default)]
    pub active: Option<u64>,
    pub epochs: u64,
    pub accounts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetiredPoolRetireLive {
    pub index: u64,
    pub tx_id: u64,
    pub retiring_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetiredPoolRetireActive {
    pub tx: u64,
    pub index: u64,
    pub retiring_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetiredPoolRetire {
    pub live: RetiredPoolRetireLive,
    pub active: RetiredPoolRetireActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetiredPoolItem {
    pub name: PoolInfo,
    pub stat: RetiredPoolItemStat,
    pub pool_retire: RetiredPoolRetire,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetiredPools {
    pub count: u64,
    pub stat: RetiredPoolStat,
    pub data: Vec<RetiredPoolItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolBirthday {
    pub live_stake: u64,
    pub delegators: u64,
    pub pledged: u64,
    pub active_epochs: u64,
    pub anniversary: String,
    pub stats: PoolStats,
    pub pool: PoolAboutPool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMultiDelegatorsStake {
    pub balance: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMultiDelegatorsItem {
    pub payment_cred: String,
    pub stake: TopMultiDelegatorsStake,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopMultiDelegators {
    pub count: u64,
    pub data: Vec<TopMultiDelegatorsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegEpochRegisteredStat {
    pub count: u64,
    pub stake: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegEpochRegisteredItem {
    pub no: u64,
    pub slot_min: u64,
    pub slot_max: u64,
    pub stat: DelegEpochRegisteredStat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeDrepsNotSpoItem {
    pub epoch_no: u64,
    pub count: u64,
    pub stake: u64,
    pub delegator: u64,
}

use crate::types::common_types::ResponseCore;

pub type PoolsListResponse = ResponseCore<PoolsList>;
pub type PoolDetailResponse = ResponseCore<PoolDetailData>;
pub type PoolRewardsResponse = ResponseCore<PoolRewards>;
pub type PoolBlocksResponse = ResponseCore<Vec<PoolBlocksData>>;
pub type PoolDelegatorsResponse = ResponseCore<PoolDelegators>;
pub type PoolUpdateResponse = ResponseCore<PoolUpdates>;
pub type PoolAwardsResponse = ResponseCore<PoolAwards>;
pub type PoolDelegatorStatsResponse = ResponseCore<PoolDelegatorStats>;
pub type PoolAboutResponse = ResponseCore<PoolAboutData>;
pub type TopMarginsWithDelegatorsResponse = ResponseCore<TopMarginsWithDelegators>;
pub type RetiredPoolsResponse = ResponseCore<RetiredPools>;
pub type PoolBirthdaysResponse = ResponseCore<Vec<PoolBirthday>>;
pub type TopMultiDelegatorsResponse = ResponseCore<TopMultiDelegators>;
pub type DelegEpochRegisteredResponse = ResponseCore<Vec<DelegEpochRegisteredItem>>;
pub type StakeDrepsNotSpoResponse = ResponseCore<Vec<StakeDrepsNotSpoItem>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolRegistrationsData {
    pub tx: Value,
    pub data: Value,
    pub block: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolRegistrationsResponseData {
    #[serde(default)]
    pub count: Option<f64>,
    pub data: Vec<PoolRegistrationsData>,
}

pub type PoolRegistrationsResponse = ResponseCore<PoolRegistrationsResponseData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepNotSpoSameTime {
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub delegator: Option<f64>,
    #[serde(default)]
    pub total: Option<Value>,
}

pub type DrepNotSpoSameTimeResponse = ResponseCore<Vec<DrepNotSpoSameTime>>;
