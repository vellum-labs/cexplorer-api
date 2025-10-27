use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::epoch_types::{
    EpochListResponse, EpochDetailParamResponse, EpochDetailStatsResponse,
};
use serde::Serialize;

/// Get list of all epochs
pub async fn get_epoch_list() -> Result<EpochListResponse, CexplorerError> {
    let endpoint = "/epoch/list";
    fetch::<EpochListResponse>(endpoint).await
}

#[derive(Debug, Serialize)]
struct EpochNoParams {
    no: u64,
}

/// Get protocol parameters for a specific epoch
pub async fn get_epoch_detail_param(no: u64) -> Result<EpochDetailParamResponse, CexplorerError> {
    let endpoint = "/epoch/param";
    let params = EpochNoParams { no };
    fetch_with_params::<EpochDetailParamResponse, EpochNoParams>(endpoint, Some(&params)).await
}

/// Get statistics for a specific epoch
pub async fn get_epoch_detail_stats(no: u64) -> Result<EpochDetailStatsResponse, CexplorerError> {
    let endpoint = "/epoch/stats";
    let params = EpochNoParams { no };
    fetch_with_params::<EpochDetailStatsResponse, EpochNoParams>(endpoint, Some(&params)).await
}