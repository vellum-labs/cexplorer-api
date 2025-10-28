use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::analytics_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EpochNoParams {
    pub epoch_no: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsAccountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drep_only: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_only: Option<u64>,
}

pub async fn get_hardforks() -> Result<HardforkResponse, CexplorerError> {
    let endpoint = "/analytics/hardforks";
    fetch::<HardforkResponse>(endpoint).await
}

pub async fn get_epoch_analytics() -> Result<EpochAnalyticsResponse, CexplorerError> {
    let endpoint = "/analytics/epoch?display=sum_fee,count_tx,avg_tx_fee,block_version,tx_composition,max_block_tx_count,count_block,count_tx_out,avg_block_size,max_block_size,count_tx_out_address,count_tx_out_stake,count_tx_out_address_not_yesterday,count_tx_out_stake_not_yesterday";
    fetch::<EpochAnalyticsResponse>(endpoint).await
}

pub async fn get_analytics_rate() -> Result<AnalyticsRateResponse, CexplorerError> {
    let endpoint = "/analytics/rate?display=sum_fee,count_tx,avg_tx_fee,block_version,tx_composition,max_block_tx_count,count_tx_out,count_block,avg_block_size,max_block_size,count_tx_out_address,count_tx_out_stake,count_tx_out_address_not_yesterday,count_tx_out_stake_not_yesterday,count_pool_relay_uniq,count_pool";
    fetch::<AnalyticsRateResponse>(endpoint).await
}

pub async fn get_analytics_pool_block(epoch_no: u64) -> Result<AnalyticsPoolBlockResponse, CexplorerError> {
    let endpoint = "/analytics/pool_block";
    let params = EpochNoParams { epoch_no };
    fetch_with_params::<AnalyticsPoolBlockResponse, EpochNoParams>(endpoint, Some(&params)).await
}

pub async fn get_analytics_staking_accounts(
    limit: Option<u64>,
    offset: Option<u64>,
    drep_only: Option<u64>,
    pool_only: Option<u64>,
) -> Result<AnalyticsTopStakingAccountsResponse, CexplorerError> {
    let endpoint = "/analytics/top_account";
    let params = AnalyticsAccountParams {
        limit,
        offset,
        drep_only,
        pool_only,
    };
    fetch_with_params::<AnalyticsTopStakingAccountsResponse, AnalyticsAccountParams>(endpoint, Some(&params)).await
}

pub async fn get_analytics_top_addresses(
    limit: Option<u64>,
    offset: Option<u64>,
    drep_only: Option<u64>,
    pool_only: Option<u64>,
) -> Result<AnalyticsTopAddressesResponse, CexplorerError> {
    let endpoint = "/analytics/top_address";
    let params = AnalyticsAccountParams {
        limit,
        offset,
        drep_only,
        pool_only,
    };
    fetch_with_params::<AnalyticsTopAddressesResponse, AnalyticsAccountParams>(endpoint, Some(&params)).await
}

pub async fn get_wealth_composition() -> Result<WealthCompositionResponse, CexplorerError> {
    let endpoint = "/analytics/wealth";
    fetch::<WealthCompositionResponse>(endpoint).await
}

pub async fn get_ada_pots() -> Result<AnalyticsAdaPotsResponse, CexplorerError> {
    let endpoint = "/analytics/ada_pot";
    fetch::<AnalyticsAdaPotsResponse>(endpoint).await
}

pub async fn get_group_list() -> Result<GroupsListResponse, CexplorerError> {
    let endpoint = "/analytics/group_list";
    fetch::<GroupsListResponse>(endpoint).await
}

pub async fn get_group_detail(id: &str) -> Result<GroupDetailResponse, CexplorerError> {
    let endpoint = format!("/analytics/group_detail?id={}", id);
    fetch::<GroupDetailResponse>(&endpoint).await
}

pub async fn get_average_pool() -> Result<AveragePoolResponse, CexplorerError> {
    let endpoint = "/analytics/avg_pool?type=avg_num_per_pool";
    fetch::<AveragePoolResponse>(endpoint).await
}
