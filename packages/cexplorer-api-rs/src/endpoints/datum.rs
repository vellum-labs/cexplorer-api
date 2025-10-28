use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::datum_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatumDetailParams {
    pub hash: String,
}

pub async fn get_datum_detail(hash: &str) -> Result<DatumDetailResponse, CexplorerError> {
    let endpoint = "/datum/detail";
    let params = DatumDetailParams {
        hash: hash.to_string(),
    };
    fetch_with_params::<DatumDetailResponse, DatumDetailParams>(endpoint, Some(&params)).await
}
