use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::token_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeFiTokenListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assetname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeFiOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_out: Option<String>,
}

pub async fn get_defi_token_list(
    limit: Option<u64>,
    offset: Option<u64>,
    sort: Option<&str>,
    order: Option<&str>,
    assetname: Option<&str>,
) -> Result<DeFiTokenListResponse, CexplorerError> {
    let endpoint = "/defi/token";
    let params = DeFiTokenListParams {
        limit,
        offset,
        order: order.map(|s| s.to_string()),
        sort: sort.map(|s| s.to_string()),
        assetname: assetname.map(|s| s.to_string()),
    };
    fetch_with_params::<DeFiTokenListResponse, DeFiTokenListParams>(endpoint, Some(&params)).await
}

pub async fn get_defi_token_stat() -> Result<DeFiTokenStatResponse, CexplorerError> {
    let endpoint = "/defi/stat";
    fetch::<DeFiTokenStatResponse>(endpoint).await
}

pub async fn get_defi_order(
    limit: Option<u64>,
    offset: Option<u64>,
    address: Option<&str>,
    stake: Option<&str>,
    status: Option<&str>,
    dex: Option<&str>,
    tx: Option<&str>,
    fingerprint: Option<&str>,
    token_in: Option<&str>,
    token_out: Option<&str>,
) -> Result<DeFiOrderListResponse, CexplorerError> {
    let endpoint = "/defi/order";
    let params = DeFiOrderParams {
        limit,
        offset,
        address: address.map(|s| s.to_string()),
        stake: stake.map(|s| s.to_string()),
        status: status.map(|s| s.to_string()),
        dex: dex.map(|s| s.to_string()),
        tx: tx.map(|s| s.to_string()),
        token: fingerprint.map(|s| s.to_string()),
        token_in: token_in.map(|s| s.to_string()),
        token_out: token_out.map(|s| s.to_string()),
    };
    fetch_with_params::<DeFiOrderListResponse, DeFiOrderParams>(endpoint, Some(&params)).await
}
