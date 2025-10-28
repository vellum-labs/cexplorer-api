use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::metadata_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataTxListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<u64>,
}

pub async fn get_metadata_tx_list(
    limit: Option<u64>,
    offset: Option<u64>,
    tx: Option<&str>,
    key: Option<u64>,
) -> Result<MetadataTxListResponse, CexplorerError> {
    let endpoint = "/metadata/list";
    let params = MetadataTxListParams {
        limit,
        offset,
        tx: tx.map(|s| s.to_string()),
        key,
    };
    fetch_with_params::<MetadataTxListResponse, MetadataTxListParams>(endpoint, Some(&params)).await
}
