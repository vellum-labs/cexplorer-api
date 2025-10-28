use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::tx_types::*;
use crate::types::drep_types::DrepRegistrationsResponse;
use crate::types::pool_types::PoolRegistrationsResponse;
use crate::types::stake_types::StakeRegistrationsResponse;
use crate::types::contract_types::ContractInteractionsResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TxListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_donation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxFilterParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "type")]
    pub filter_type: String,
}

pub async fn get_tx_detail(hash: &str) -> Result<TxDetailResponse, CexplorerError> {
    let endpoint = format!("/tx/detail?hash={}", hash);
    fetch::<TxDetailResponse>(&endpoint).await
}

pub async fn get_tx_list(
    hash: Option<&str>,
    limit: Option<u64>,
    offset: Option<u64>,
    address: Option<&str>,
    stake: Option<&str>,
    asset: Option<&str>,
    script: Option<&str>,
    has_donation: Option<bool>,
    policy: Option<&str>,
) -> Result<TxListResponse, CexplorerError> {
    let endpoint = "/tx/list";
    let params = TxListParams {
        hash: hash.map(|s| s.to_string()),
        limit,
        offset,
        address: address.map(|s| s.to_string()),
        stake: stake.map(|s| s.to_string()),
        asset: asset.map(|s| s.to_string()),
        script: script.map(|s| s.to_string()),
        has_donation,
        policy: policy.map(|s| s.to_string()),
    };
    fetch_with_params::<TxListResponse, TxListParams>(endpoint, Some(&params)).await
}

pub async fn get_drep_registrations(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<DrepRegistrationsResponse, CexplorerError> {
    let endpoint = "/tx/filter";
    let params = TxFilterParams {
        limit,
        offset,
        filter_type: "drep_registrations".to_string(),
    };
    fetch_with_params::<DrepRegistrationsResponse, TxFilterParams>(endpoint, Some(&params)).await
}

pub async fn get_drep_deregistrations(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<DrepRegistrationsResponse, CexplorerError> {
    let endpoint = "/tx/filter";
    let params = TxFilterParams {
        limit,
        offset,
        filter_type: "drep_deregistrations".to_string(),
    };
    fetch_with_params::<DrepRegistrationsResponse, TxFilterParams>(endpoint, Some(&params)).await
}

pub async fn get_drep_updates(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<DrepRegistrationsResponse, CexplorerError> {
    let endpoint = "/tx/filter";
    let params = TxFilterParams {
        limit,
        offset,
        filter_type: "drep_updates".to_string(),
    };
    fetch_with_params::<DrepRegistrationsResponse, TxFilterParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_registrations(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<PoolRegistrationsResponse, CexplorerError> {
    let endpoint = "/tx/filter";
    let params = TxFilterParams {
        limit,
        offset,
        filter_type: "pool_registrations".to_string(),
    };
    fetch_with_params::<PoolRegistrationsResponse, TxFilterParams>(endpoint, Some(&params)).await
}

pub async fn get_pool_deregistrations(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<PoolRegistrationsResponse, CexplorerError> {
    let endpoint = "/tx/filter";
    let params = TxFilterParams {
        limit,
        offset,
        filter_type: "pool_deregistrations".to_string(),
    };
    fetch_with_params::<PoolRegistrationsResponse, TxFilterParams>(endpoint, Some(&params)).await
}

pub async fn get_stake_registrations(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<StakeRegistrationsResponse, CexplorerError> {
    let endpoint = "/tx/filter";
    let params = TxFilterParams {
        limit,
        offset,
        filter_type: "stake_key_registrations".to_string(),
    };
    fetch_with_params::<StakeRegistrationsResponse, TxFilterParams>(endpoint, Some(&params)).await
}

pub async fn get_contract_transactions(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<ContractInteractionsResponse, CexplorerError> {
    let endpoint = "/tx/filter";
    let params = TxFilterParams {
        limit,
        offset,
        filter_type: "contract_transactions".to_string(),
    };
    fetch_with_params::<ContractInteractionsResponse, TxFilterParams>(endpoint, Some(&params)).await
}
