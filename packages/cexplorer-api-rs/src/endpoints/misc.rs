use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::misc_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MiscMarketParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_no: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiscSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiscValidateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub ident: String,
}

pub async fn get_misc_api() -> Result<MiscApiResponse, CexplorerError> {
    let endpoint = "/misc/api";
    fetch::<MiscApiResponse>(endpoint).await
}

pub async fn get_misc_basic() -> Result<MiscBasicResponse, CexplorerError> {
    let endpoint = "/misc/basic";
    fetch::<MiscBasicResponse>(endpoint).await
}

pub async fn get_misc_rate() -> Result<MiscRateResponse, CexplorerError> {
    let endpoint = "/misc/rate";
    fetch::<MiscRateResponse>(endpoint).await
}

pub async fn get_misc_const() -> Result<MiscConstResponse, CexplorerError> {
    let endpoint = "/misc/const";
    fetch::<MiscConstResponse>(endpoint).await
}

pub async fn get_misc_market(
    epoch_no: Option<u64>,
    date: Option<&str>,
) -> Result<MiscMarketResponse, CexplorerError> {
    let endpoint = "/misc/market";
    let params = MiscMarketParams {
        epoch_no,
        date: date.map(|s| s.to_string()),
    };
    fetch_with_params::<MiscMarketResponse, MiscMarketParams>(endpoint, Some(&params)).await
}

pub async fn get_misc_search(
    query: Option<&str>,
    category: Option<&str>,
    locale: Option<&str>,
) -> Result<MiscSearchResponse, CexplorerError> {
    let endpoint = "/misc/search";
    let params = MiscSearchParams {
        query: query.map(|s| s.to_string()),
        category: category.map(|s| s.to_string()),
        lng: locale.map(|s| s.to_string()),
    };
    fetch_with_params::<MiscSearchResponse, MiscSearchParams>(endpoint, Some(&params)).await
}

pub async fn get_poll_list() -> Result<PollListResponse, CexplorerError> {
    let endpoint = "/misc/gw/gov";
    fetch::<PollListResponse>(endpoint).await
}

pub async fn misc_validate(
    validate_type: Option<&str>,
    ident: &str,
) -> Result<MiscValidateResponse, CexplorerError> {
    let endpoint = "/misc/validate";
    let params = MiscValidateParams {
        r#type: validate_type.map(|s| s.to_string()),
        ident: ident.to_string(),
    };
    fetch_with_params::<MiscValidateResponse, MiscValidateParams>(endpoint, Some(&params)).await
}

pub async fn get_misc_health() -> Result<MiscHealthResponse, CexplorerError> {
    let endpoint = "/misc/health";
    fetch::<MiscHealthResponse>(endpoint).await
}

pub async fn get_misc_protocol_parameters() -> Result<MiscProtocolParametersResponse, CexplorerError> {
    let endpoint = "/misc/protocol_parameters";
    fetch::<MiscProtocolParametersResponse>(endpoint).await
}
