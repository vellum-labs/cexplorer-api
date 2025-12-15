use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::tool_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TxSentParams {
    pub id: String,
    pub r#type: String,
    pub campaign: String,
}

/// Send delegation or donation transaction information
pub async fn send_tx_sent(
    hash: &str,
    pool_id: &str,
    tx_type: &str,
) -> Result<TxSentResponse, CexplorerError> {
    let endpoint = "/tool/tx_sent";
    let params = TxSentParams {
        id: hash.to_string(),
        r#type: tx_type.to_string(),
        campaign: pool_id.to_string(),
    };
    fetch_with_params::<TxSentResponse, TxSentParams>(endpoint, Some(&params)).await
}
