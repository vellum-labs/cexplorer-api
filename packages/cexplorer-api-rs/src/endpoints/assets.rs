use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::assets_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchlist_only: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetDetailParams {
    pub fingerprint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetOwnerParams {
    pub assetname: String,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMetadataParams {
    pub assetname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetMintParams {
    pub assetname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assetname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

pub async fn get_asset_list(
    limit: Option<u64>,
    offset: Option<u64>,
    filter: Option<&str>,
    name: Option<&str>,
    order: Option<&str>,
    policy: Option<&str>,
    sort: Option<&str>,
    watchlist: Option<&str>,
) -> Result<AssetListResponse, CexplorerError> {
    let endpoint = "/asset/list";
    let params = AssetListParams {
        limit,
        offset,
        sort: sort.map(|s| s.to_string()),
        order: order.map(|s| s.to_string()),
        policy: policy.map(|s| s.to_string()),
        name: name.map(|s| s.to_string()),
        filter: filter.map(|s| s.to_string()),
        watchlist_only: watchlist.map(|s| s.to_string()),
    };
    fetch_with_params::<AssetListResponse, AssetListParams>(endpoint, Some(&params)).await
}

pub async fn get_asset_detail(fingerprint: &str) -> Result<AssetDetailResponse, CexplorerError> {
    let endpoint = "/asset/detail";
    let params = AssetDetailParams {
        fingerprint: fingerprint.to_string(),
    };
    fetch_with_params::<AssetDetailResponse, AssetDetailParams>(endpoint, Some(&params)).await
}

pub async fn get_asset_owners(
    assetname: &str,
    offset: u64,
    limit: u64,
) -> Result<AssetOwnersNftResponse, CexplorerError> {
    let endpoint = "/asset/owner";
    let params = AssetOwnerParams {
        assetname: assetname.to_string(),
        offset,
        limit,
    };
    fetch_with_params::<AssetOwnersNftResponse, AssetOwnerParams>(endpoint, Some(&params)).await
}

pub async fn get_nft_asset_owners(
    assetname: &str,
    offset: u64,
    limit: u64,
) -> Result<AssetOwnersNftResponse, CexplorerError> {
    let endpoint = "/asset/owner_history";
    let params = AssetOwnerParams {
        assetname: assetname.to_string(),
        offset,
        limit,
    };
    fetch_with_params::<AssetOwnersNftResponse, AssetOwnerParams>(endpoint, Some(&params)).await
}

pub async fn get_asset_metadata(assetname: &str) -> Result<AssetMetadataResponse, CexplorerError> {
    let endpoint = "/asset/metadata";
    let params = AssetMetadataParams {
        assetname: assetname.to_string(),
    };
    fetch_with_params::<AssetMetadataResponse, AssetMetadataParams>(endpoint, Some(&params)).await
}

pub async fn get_asset_mint(
    assetname: &str,
    id: Option<&str>,
) -> Result<AssetMintResponse, CexplorerError> {
    let endpoint = "/policy/mint";
    let params = AssetMintParams {
        assetname: assetname.to_string(),
        id: id.map(|s| s.to_string()),
    };
    fetch_with_params::<AssetMintResponse, AssetMintParams>(endpoint, Some(&params)).await
}

pub async fn get_asset_stats(
    assetname: Option<&str>,
    fingerprint: Option<&str>,
) -> Result<AssetStatsResponse, CexplorerError> {
    let endpoint = "/asset/stat";
    let params = AssetStatsParams {
        assetname: assetname.map(|s| s.to_string()),
        fingerprint: fingerprint.map(|s| s.to_string()),
    };
    fetch_with_params::<AssetStatsResponse, AssetStatsParams>(endpoint, Some(&params)).await
}
