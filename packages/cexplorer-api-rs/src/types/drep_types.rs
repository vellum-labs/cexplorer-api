use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::tx_types::{TxBasicInfo, BlockBasicInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepCount {
    #[serde(default)]
    pub total: Option<f64>,
    #[serde(default)]
    pub active: Option<f64>,
    #[serde(default)]
    pub inactive: Option<f64>,
    #[serde(default)]
    pub retired: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepDistr {
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub delegators: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub active_until: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatDrep {
    #[serde(default)]
    pub count: Option<DrepCount>,
    #[serde(default)]
    pub distr: Option<DrepDistr>,
    #[serde(default)]
    pub deposit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepStake {
    #[serde(default)]
    pub total: Option<f64>,
    #[serde(default)]
    pub drep_always_abstain: Option<f64>,
    #[serde(default)]
    pub drep_always_no_confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepCommitteeCount {
    #[serde(default)]
    pub total: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepCommittee {
    #[serde(default)]
    pub count: Option<DrepCommitteeCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepStatOverall {
    #[serde(default)]
    pub drep: Option<StatDrep>,
    #[serde(default)]
    pub stake: Option<DrepStake>,
    #[serde(default)]
    pub committee: Option<DrepCommittee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepHash {
    pub raw: String,
    pub view: String,
    #[serde(default)]
    pub has_script: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToplistItem {
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub hash: Option<DrepHash>,
    #[serde(default)]
    pub distr: Option<DrepDistr>,
    #[serde(default)]
    pub since: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepToplist {
    #[serde(default)]
    pub by_stake: Option<Vec<ToplistItem>>,
    #[serde(default)]
    pub by_count: Option<Vec<ToplistItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatorItem {
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepDelegator {
    #[serde(default)]
    pub total: Option<DelegatorItem>,
    #[serde(default)]
    pub drep_always_abstain: Option<DelegatorItem>,
    #[serde(default)]
    pub drep_always_no_confidence: Option<DelegatorItem>,
    #[serde(default)]
    pub delegated_stake_pools: Option<DelegatorItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDrepDistr {
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub drep_always_abstain: Option<f64>,
    #[serde(default)]
    pub drep_always_no_confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepAnalytics {
    #[serde(default)]
    pub toplist: Option<DrepToplist>,
    #[serde(default)]
    pub delegator: Option<DrepDelegator>,
    #[serde(default)]
    pub drep_distr: Option<Vec<AnalyticsDrepDistr>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeDrepRetiredDrep {
    #[serde(default)]
    pub count: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeDrepRetiredDelegator {
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeDrepRetired {
    #[serde(default)]
    pub drep: Option<StakeDrepRetiredDrep>,
    #[serde(default)]
    pub delegator: Option<StakeDrepRetiredDelegator>,
}

pub type StakeDrepRetiredResponse = ResponseCore<StakeDrepRetired>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepVotes {
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub vote: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepTotal {
    #[serde(default)]
    pub votes: Option<Vec<DrepVotes>>,
    #[serde(default)]
    pub opportunity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepListStat {
    #[serde(default)]
    pub total: Option<DrepTotal>,
    #[serde(default)]
    pub recently: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepOwner {
    pub stake: String,
    pub address: String,
    #[serde(default)]
    pub balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepTopDelegator {
    pub view: String,
    #[serde(default)]
    pub stake: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepGovAction {
    #[serde(default)]
    pub vote: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepPoolIdent {
    pub ident: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepListData {
    #[serde(default)]
    pub is_active: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub data: Option<Value>,
    #[serde(default)]
    pub hash: Option<DrepHash>,
    #[serde(default)]
    pub distr: Option<DrepDistr>,
    #[serde(default)]
    pub stat: Option<DrepListStat>,
    #[serde(default)]
    pub since: Option<String>,
    #[serde(default)]
    pub owner: Option<DrepOwner>,
    #[serde(default)]
    pub gov_action: Option<DrepGovAction>,
    #[serde(default)]
    pub pool: Option<Vec<DrepPoolIdent>>,
    #[serde(default)]
    pub top_delegator: Option<DrepTopDelegator>,
    #[serde(default)]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepTx {
    #[serde(default)]
    pub slot: Option<f64>,
    #[serde(default)]
    pub epoch_no: Option<f64>,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepCertItem {
    #[serde(default)]
    pub deposit: Option<f64>,
    pub tx: DrepTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepCert {
    pub update: DrepCertItem,
    pub registration: DrepCertItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepActionTx {
    pub hash: String,
    pub time: String,
    #[serde(default)]
    pub invalid_hereafter: Option<String>,
    #[serde(default)]
    pub treasury_donation: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub tx: DrepActionTx,
    #[serde(default)]
    pub vote: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepGovActionDetail {
    #[serde(default)]
    pub total: Option<f64>,
    #[serde(default)]
    pub active: Option<f64>,
    #[serde(default)]
    pub enacted: Option<f64>,
    #[serde(default)]
    pub expires: Option<f64>,
    #[serde(default)]
    pub ratified: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepStat {
    #[serde(default)]
    pub total: Option<DrepTotal>,
    #[serde(default)]
    pub recently: Option<String>,
    #[serde(default)]
    pub gov_action: Option<Vec<DrepGovActionDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepDetailData {
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub objectives: Option<String>,
    #[serde(default)]
    pub motivations: Option<String>,
    #[serde(default)]
    pub qualifications: Option<String>,
    #[serde(default)]
    pub payment_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepDetail {
    #[serde(default)]
    pub deposit: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub is_active: Option<bool>,
    #[serde(default)]
    pub data: Option<DrepDetailData>,
    #[serde(default)]
    pub cert: Option<DrepCert>,
    #[serde(default)]
    pub hash: Option<DrepHash>,
    #[serde(default)]
    pub distr: Option<DrepDistr>,
    #[serde(default)]
    pub action: Option<Vec<DrepAction>>,
    #[serde(default)]
    pub stat: Option<DrepStat>,
    #[serde(default)]
    pub since: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepInfo {
    pub id: String,
    #[serde(default)]
    pub meta: Option<Value>,
    #[serde(default)]
    pub power: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatorInfo {
    pub id: String,
    #[serde(default)]
    pub data: Option<Value>,
    pub tx: DrepTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepAnchorOffchain {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepAnchor {
    pub url: String,
    pub data_hash: String,
    #[serde(default)]
    pub offchain: Option<DrepAnchorOffchain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepProposalIdent {
    pub id: String,
    pub bech: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepProposalTx {
    pub hash: String,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepProposalDescription {
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepProposal {
    pub ident: DrepProposalIdent,
    pub tx: DrepProposalTx,
    #[serde(rename = "type")]
    pub proposal_type: String,
    pub anchor: DrepAnchor,
    #[serde(default)]
    pub deposit: Option<f64>,
    #[serde(default)]
    pub expiration: Option<f64>,
    pub description: DrepProposalDescription,
    #[serde(default)]
    pub previous: Option<Value>,
    #[serde(default)]
    pub ratified_epoch: Option<f64>,
    #[serde(default)]
    pub enacted_epoch: Option<f64>,
    #[serde(default)]
    pub dropped_epoch: Option<f64>,
    #[serde(default)]
    pub expired_epoch: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepVoteItem {
    pub voter_role: String,
    pub vote: String,
    pub proposal: DrepProposal,
    pub info: DrepInfo,
    pub tx: DrepActionTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatorData {
    pub view: String,
    #[serde(default)]
    pub slot_first_registered: Option<f64>,
    #[serde(default)]
    pub slot_update: Option<f64>,
    pub script: String,
    #[serde(default)]
    pub live_stake: Option<f64>,
    pub live_drep: DelegatorInfo,
    #[serde(default)]
    pub previous_drep: Option<DelegatorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepRegistrationsDataInfo {
    pub raw: String,
    pub view: String,
    #[serde(default)]
    pub deposit: Option<f64>,
    #[serde(default)]
    pub has_script: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepRegistrationsData {
    pub tx: TxBasicInfo,
    pub data: DrepRegistrationsDataInfo,
    pub block: BlockBasicInfo,
    pub owner: DrepOwner,
}

pub type DrepRegistrationsResponse = ResponseCore<Vec<DrepRegistrationsData>>;
pub type DrepStatResponse = ResponseCore<DrepStat>;
pub type DrepAnalyticsResponse = ResponseCore<DrepAnalytics>;
pub type DrepListResponse = ResponseCore<Vec<DrepListData>>;
pub type DrepDetailResponse = ResponseCore<DrepDetail>;
pub type DrepVoteResponse = ResponseCore<Vec<DrepVoteItem>>;
pub type DrepDelegatorResponse = ResponseCore<Vec<DelegatorData>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AverageDrep {
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub avg_delegator: Option<f64>,
    #[serde(default)]
    pub avg_epoch_stake: Option<f64>,
}

pub type AverageDrepResponse = ResponseCore<Vec<AverageDrep>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrepSpoSameTime {
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub delegator: Option<f64>,
}

pub type DrepSpoSameTimeResponse = ResponseCore<Vec<DrepSpoSameTime>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeIsSpoDrep {
    #[serde(default)]
    pub epoch_no: Option<f64>,
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub delegator: Option<f64>,
}

pub type StakeIsSpoDrepResponse = ResponseCore<Vec<StakeIsSpoDrep>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegEpochChangesStat {
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegEpochChanges {
    #[serde(default)]
    pub no: Option<f64>,
    #[serde(default)]
    pub slot_min: Option<f64>,
    #[serde(default)]
    pub slot_max: Option<f64>,
    #[serde(default)]
    pub stat: Option<DelegEpochChangesStat>,
}

pub type DelegEpochChangesResponse = ResponseCore<Vec<DelegEpochChanges>>;
