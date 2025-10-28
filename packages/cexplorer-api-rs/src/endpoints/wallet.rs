use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::wallet_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareWalletsParams {
    pub lng: String,
    #[serde(rename = "type")]
    pub wallet_type: String,
    pub url: String,
}

pub async fn compare_wallets() -> Result<CompareWalletsResponse, CexplorerError> {
    let endpoint = "/article/detail";
    let params = CompareWalletsParams {
        lng: "en".to_string(),
        wallet_type: "page".to_string(),
        url: "wallets".to_string(),
    };
    fetch_with_params::<CompareWalletsResponse, CompareWalletsParams>(endpoint, Some(&params)).await
}
