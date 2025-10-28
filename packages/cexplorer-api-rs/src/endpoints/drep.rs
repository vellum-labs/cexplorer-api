use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::drep_types::*;
use crate::types::pool_types::{PoolDelegatorStatsResponse, DrepNotSpoSameTimeResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DrepListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchlist_only: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gov_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_spo: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_spo: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrepVoteParams {
    pub voter_role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrepDelegatorParams {
    pub view: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewParams {
    pub view: String,
}

pub async fn get_drep_stat() -> Result<DrepStatResponse, CexplorerError> {
    let endpoint = "/gov/stat";
    fetch::<DrepStatResponse>(endpoint).await
}

pub async fn get_drep_analytics() -> Result<DrepAnalyticsResponse, CexplorerError> {
    let endpoint = "/gov/drep_analytics";
    fetch::<DrepAnalyticsResponse>(endpoint).await
}

pub async fn get_stake_drep_retired() -> Result<StakeDrepRetiredResponse, CexplorerError> {
    let endpoint = "/analytics/stake?type=stake_drep_retired";
    fetch::<StakeDrepRetiredResponse>(endpoint).await
}

pub async fn get_drep_list(
    limit: Option<u64>,
    offset: Option<u64>,
    sort: Option<&str>,
    order: Option<&str>,
    view: Option<&str>,
    watchlist_only: Option<&str>,
    gov_action: Option<&str>,
    is_spo: Option<u64>,
    is_not_spo: Option<u64>,
) -> Result<DrepListResponse, CexplorerError> {
    let endpoint = "/gov/drep_list";
    let params = DrepListParams {
        limit,
        offset,
        view: view.map(|s| s.to_string()),
        watchlist_only: watchlist_only.map(|s| s.to_string()),
        sort: sort.map(|s| s.to_string()),
        order: order.map(|s| s.to_string()),
        gov_action: gov_action.map(|s| s.to_string()),
        is_spo,
        is_not_spo,
    };
    fetch_with_params::<DrepListResponse, DrepListParams>(endpoint, Some(&params)).await
}

pub async fn get_drep_detail(hash: &str) -> Result<DrepDetailResponse, CexplorerError> {
    let endpoint = format!("/gov/drep_detail?view={}", hash);
    fetch::<DrepDetailResponse>(&endpoint).await
}

pub async fn get_drep_vote(
    voter_role: &str,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<DrepVoteResponse, CexplorerError> {
    let endpoint = "/gov/vote";
    let params = DrepVoteParams {
        voter_role: voter_role.to_string(),
        limit,
        offset,
    };
    fetch_with_params::<DrepVoteResponse, DrepVoteParams>(endpoint, Some(&params)).await
}

pub async fn get_drep_delegator(
    view: &str,
    limit: Option<u64>,
    offset: Option<u64>,
    filter: Option<&str>,
    order: Option<&str>,
) -> Result<DrepDelegatorResponse, CexplorerError> {
    let endpoint = "/gov/drep_delegator";
    let params = DrepDelegatorParams {
        view: view.to_string(),
        limit,
        offset,
        order: order.map(|s| s.to_string()),
        filter: filter.map(|s| s.to_string()),
    };
    fetch_with_params::<DrepDelegatorResponse, DrepDelegatorParams>(endpoint, Some(&params)).await
}

pub async fn get_drep_delegator_stats(view: &str) -> Result<PoolDelegatorStatsResponse, CexplorerError> {
    let endpoint = "/gov/drep_delegator_stats";
    let params = ViewParams {
        view: view.to_string(),
    };
    fetch_with_params::<PoolDelegatorStatsResponse, ViewParams>(endpoint, Some(&params)).await
}

pub async fn get_average_drep() -> Result<AverageDrepResponse, CexplorerError> {
    let endpoint = "/analytics/avg_drep?type=avg_num_per_drep";
    fetch::<AverageDrepResponse>(endpoint).await
}

pub async fn get_drep_spo_same_time() -> Result<DrepSpoSameTimeResponse, CexplorerError> {
    let endpoint = "/analytics/drep_spo?type=power_drep_spo_same_time";
    fetch::<DrepSpoSameTimeResponse>(endpoint).await
}

pub async fn get_stake_is_spo_drep() -> Result<StakeIsSpoDrepResponse, CexplorerError> {
    let endpoint = "/analytics/stake?type=stake_is_spo_drep";
    fetch::<StakeIsSpoDrepResponse>(endpoint).await
}

pub async fn get_drep_not_spo_same_time() -> Result<DrepNotSpoSameTimeResponse, CexplorerError> {
    let endpoint = "/analytics/drep_spo?type=power_drep_not_spo";
    fetch::<DrepNotSpoSameTimeResponse>(endpoint).await
}

pub async fn get_deleg_epoch_changes() -> Result<DelegEpochChangesResponse, CexplorerError> {
    let endpoint = "/analytics/deleg?type=deleg_epoch_changes";
    fetch::<DelegEpochChangesResponse>(endpoint).await
}
