use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::script_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptHashParams {
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptRedeemerParams {
    pub hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

pub async fn get_script_detail(hash: &str) -> Result<ScriptDetailResponse, CexplorerError> {
    let endpoint = "/script/detail";
    let params = ScriptHashParams {
        hash: hash.to_string(),
    };
    fetch_with_params::<ScriptDetailResponse, ScriptHashParams>(endpoint, Some(&params)).await
}

pub async fn get_script_detail_redeemer(
    hash: &str,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<ScriptDetailRedeemerResponse, CexplorerError> {
    let endpoint = "/script/detail_redeemer";
    let params = ScriptRedeemerParams {
        hash: hash.to_string(),
        limit,
        offset,
    };
    fetch_with_params::<ScriptDetailRedeemerResponse, ScriptRedeemerParams>(endpoint, Some(&params)).await
}

pub async fn get_script_list(
    limit: Option<u64>,
    offset: Option<u64>,
    hash: Option<&str>,
    order: Option<&str>,
) -> Result<ScriptListResponse, CexplorerError> {
    let endpoint = "/script/list";
    let params = ScriptListParams {
        limit,
        offset,
        hash: hash.map(|s| s.to_string()),
        order: order.map(|s| s.to_string()),
    };
    fetch_with_params::<ScriptListResponse, ScriptListParams>(endpoint, Some(&params)).await
}
