use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::delegation_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DelegationParams {
    pub view: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelegationToRetiredParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

pub async fn get_delegations_state(view: &str) -> Result<DelegationStateResponse, CexplorerError> {
    let endpoint = format!("/account/delegation_state?view={}", view);
    fetch::<DelegationStateResponse>(&endpoint).await
}

pub async fn get_stake_delegations(
    view: &str,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<DelegationResponse, CexplorerError> {
    let endpoint = "/account/delegation";
    let params = DelegationParams {
        view: view.to_string(),
        limit,
        offset,
    };
    fetch_with_params::<DelegationResponse, DelegationParams>(endpoint, Some(&params)).await
}

pub async fn get_delegations_to_retired(
    delegation_type: Option<&str>,
    limit: Option<u64>,
    offset: Option<u64>,
    order: Option<&str>,
) -> Result<DelegationToRetiredResponse, CexplorerError> {
    let endpoint = "/account/delegation_to_retired";
    let params = DelegationToRetiredParams {
        r#type: delegation_type.map(|s| s.to_string()),
        limit,
        offset,
        order: order.map(|s| s.to_string()),
    };
    fetch_with_params::<DelegationToRetiredResponse, DelegationToRetiredParams>(endpoint, Some(&params)).await
}
