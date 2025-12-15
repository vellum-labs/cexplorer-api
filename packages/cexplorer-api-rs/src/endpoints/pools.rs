use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::pool_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolIdParams {
    pub pool_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gov_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_drep: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_not_drep: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolDetailParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_raw: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolDelegatorsParams {
    pub pool_id: String,
    #[serde(rename = "type")]
    pub delegator_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolRewardsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetiredPoolsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub retired_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopPoolParams {
    #[serde(rename = "type")]
    pub pool_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitOffsetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

pub async fn get_pool_blocks(pool_id: &str) -> Result<PoolBlocksResponse, CexplorerError> {
    let endpoint = "/pool/block";
    let params = PoolIdParams { pool_id: pool_id.to_string() };
    fetch_with_params::<PoolBlocksResponse, PoolIdParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_delegators(
    pool_id: &str,
    delegator_type: &str,
    limit: Option<u64>,
    offset: Option<u64>,
    sort: Option<&str>,
    order: Option<&str>,
) -> Result<PoolDelegatorsResponse, CexplorerError> {
    let endpoint = "/pool/delegator";
    let params = PoolDelegatorsParams {
        pool_id: pool_id.to_string(),
        delegator_type: delegator_type.to_string(),
        limit,
        offset,
        sort: sort.map(|s| s.to_string()),
        order: order.map(|s| s.to_string()),
    };
    fetch_with_params::<PoolDelegatorsResponse, PoolDelegatorsParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_reward(
    limit: Option<u64>,
    offset: Option<u64>,
    name: Option<&str>,
    pool_id: Option<&str>,
) -> Result<PoolRewardsResponse, CexplorerError> {
    let endpoint = "/pool/reward";
    let params = PoolRewardsParams {
        limit,
        offset,
        name: name.map(|s| s.to_string()),
        pool_id: pool_id.map(|s| s.to_string()),
    };
    fetch_with_params::<PoolRewardsResponse, PoolRewardsParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_detail(
    pool_id: Option<&str>,
    hash_raw: Option<&str>,
) -> Result<PoolDetailResponse, CexplorerError> {
    let endpoint = "/pool/detail";
    let params = PoolDetailParams {
        pool_id: pool_id.map(|s| s.to_string()),
        hash_raw: hash_raw.map(|s| s.to_string()),
    };
    fetch_with_params::<PoolDetailResponse, PoolDetailParams>(endpoint, Some(&params)).await
}

pub async fn get_pools_list(
    limit: Option<u64>,
    offset: Option<u64>,
    sort: Option<&str>,
    order: Option<&str>,
    name: Option<&str>,
    pool_id: Option<&str>,
    gov_action: Option<&str>,
    is_drep: Option<u64>,
    is_not_drep: Option<u64>,
) -> Result<PoolsListResponse, CexplorerError> {
    let endpoint = "/pool/list";
    let params = PoolListParams {
        limit,
        offset,
        sort: sort.map(|s| s.to_string()),
        order: order.map(|s| s.to_string()),
        name: name.map(|s| s.to_string()),
        pool_id: pool_id.map(|s| s.to_string()),
        gov_action: gov_action.map(|s| s.to_string()),
        is_drep,
        is_not_drep,
    };
    fetch_with_params::<PoolsListResponse, PoolListParams>(endpoint, Some(&params)).await
}

pub async fn get_pools_birthdays(pool_id: &str) -> Result<PoolBirthdaysResponse, CexplorerError> {
    let endpoint = "/pool/birthday";
    let params = PoolIdParams { pool_id: pool_id.to_string() };
    fetch_with_params::<PoolBirthdaysResponse, PoolIdParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_update(pool_id: &str) -> Result<PoolUpdateResponse, CexplorerError> {
    let endpoint = "/pool/update";
    let params = PoolIdParams { pool_id: pool_id.to_string() };
    fetch_with_params::<PoolUpdateResponse, PoolIdParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_awards(pool_id: &str) -> Result<PoolAwardsResponse, CexplorerError> {
    let endpoint = "/pool/award";
    let params = PoolIdParams { pool_id: pool_id.to_string() };
    fetch_with_params::<PoolAwardsResponse, PoolIdParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_delegators_stats(pool_id: &str) -> Result<PoolDelegatorStatsResponse, CexplorerError> {
    let endpoint = "/pool/delegator_stats";
    let params = PoolIdParams { pool_id: pool_id.to_string() };
    fetch_with_params::<PoolDelegatorStatsResponse, PoolIdParams>(endpoint, Some(&params)).await
}

pub async fn get_global_pool_awards(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<PoolAwardsResponse, CexplorerError> {
    let endpoint = "/pool/award";
    let params = LimitOffsetParams { limit, offset };
    fetch_with_params::<PoolAwardsResponse, LimitOffsetParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_about(pool_id: &str) -> Result<PoolAboutResponse, CexplorerError> {
    let endpoint = "/pool/about";
    let params = PoolIdParams { pool_id: pool_id.to_string() };
    fetch_with_params::<PoolAboutResponse, PoolIdParams>(endpoint, Some(&params)).await
}

pub async fn get_top_margins_with_delegators(
    pool_type: &str,
    offset: Option<u64>,
    limit: Option<u64>,
) -> Result<TopMarginsWithDelegatorsResponse, CexplorerError> {
    let endpoint = "/analytics/top_pool";
    let params = TopPoolParams {
        pool_type: pool_type.to_string(),
        offset,
        limit,
    };
    fetch_with_params::<TopMarginsWithDelegatorsResponse, TopPoolParams>(endpoint, Some(&params)).await
}

pub async fn get_retired_pools(
    retired_type: Option<&str>,
    limit: Option<u64>,
    offset: Option<u64>,
    order: Option<&str>,
) -> Result<RetiredPoolsResponse, CexplorerError> {
    let endpoint = "/pool/retired";
    let params = RetiredPoolsParams {
        retired_type: retired_type.map(|s| s.to_string()),
        limit,
        offset,
        order: order.map(|s| s.to_string()),
    };
    fetch_with_params::<RetiredPoolsResponse, RetiredPoolsParams>(endpoint, Some(&params)).await
}

pub async fn get_top_multi_delegators(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<TopMultiDelegatorsResponse, CexplorerError> {
    let endpoint = "/analytics/top_multi";
    let params = LimitOffsetParams { limit, offset };
    fetch_with_params::<TopMultiDelegatorsResponse, LimitOffsetParams>(endpoint, Some(&params)).await
}

pub async fn get_deleg_epoch_registered() -> Result<DelegEpochRegisteredResponse, CexplorerError> {
    let endpoint = "/analytics/deleg?type=deleg_epoch_registered";
    fetch::<DelegEpochRegisteredResponse>(endpoint).await
}

pub async fn get_stake_dreps_not_spo() -> Result<StakeDrepsNotSpoResponse, CexplorerError> {
    let endpoint = "/analytics/stake?type=stake_dreps_not_spo";
    fetch::<StakeDrepsNotSpoResponse>(endpoint).await
}

pub async fn get_pool_retire(pool_id: &str) -> Result<PoolRetireResponse, CexplorerError> {
    let endpoint = "/pool/retire";
    let params = PoolIdParams {
        pool_id: pool_id.to_string(),
    };
    fetch_with_params::<PoolRetireResponse, PoolIdParams>(endpoint, Some(&params)).await
}
