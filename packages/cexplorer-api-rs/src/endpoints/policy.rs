use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::policy_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyIdParams {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyOwnerParams {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

pub async fn get_policy_detail(id: &str) -> Result<PolicyDetailResponse, CexplorerError> {
    let endpoint = "/policy/detail";
    let params = PolicyIdParams {
        id: id.to_string(),
    };
    fetch_with_params::<PolicyDetailResponse, PolicyIdParams>(endpoint, Some(&params)).await
}

pub async fn get_policy_stats(id: &str) -> Result<PolicyStatsResponse, CexplorerError> {
    let endpoint = "/policy/stat";
    let params = PolicyIdParams {
        id: id.to_string(),
    };
    fetch_with_params::<PolicyStatsResponse, PolicyIdParams>(endpoint, Some(&params)).await
}

pub async fn get_policy_owner(
    id: &str,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<PolicyOwnerResponse, CexplorerError> {
    let endpoint = "/policy/owner";
    let params = PolicyOwnerParams {
        id: id.to_string(),
        limit,
        offset,
    };
    fetch_with_params::<PolicyOwnerResponse, PolicyOwnerParams>(endpoint, Some(&params)).await
}
