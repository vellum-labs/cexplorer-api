use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::{BlockDetailResponse, BlocksListResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BlockListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_no: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_no: Option<u64>,
}

pub async fn get_block_list(params: BlockListParams) -> Result<BlocksListResponse, CexplorerError> {
    let endpoint = "/block/list";
    fetch_with_params::<BlocksListResponse, BlockListParams>(endpoint, Some(&params)).await
}

pub async fn get_block_detail(hash: &str) -> Result<BlockDetailResponse, CexplorerError> {
    let endpoint = format!("/block/detail?hash={}", hash);
    fetch::<BlockDetailResponse>(&endpoint).await
}