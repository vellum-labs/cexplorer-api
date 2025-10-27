use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::{
    AddressDetailResponse, AddressDetailUTXOResponse,
    AddressListResponse, AddressInspectorResponse,
};
use serde::Serialize;

/// Get detailed information for a specific address
pub async fn get_address_detail(view: &str) -> Result<AddressDetailResponse, CexplorerError> {
    let endpoint = format!("/address/detail?view={}", view);
    fetch::<AddressDetailResponse>(&endpoint).await
}

#[derive(Debug, Serialize)]
pub struct AddressListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_cred: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchlist_only: Option<String>,
}

/// Get a list of addresses based on filters
pub async fn get_address_list(params: AddressListParams) -> Result<AddressListResponse, CexplorerError> {
    let endpoint = "/address/list";
    fetch_with_params::<AddressListResponse, AddressListParams>(endpoint, Some(&params)).await
}

/// Get UTXOs (Unspent Transaction Outputs) for a given address
pub async fn get_address_utxo(view: &str) -> Result<AddressDetailUTXOResponse, CexplorerError> {
    let endpoint = format!("/address/utxo?view={}", view);
    fetch::<AddressDetailUTXOResponse>(&endpoint).await
}

/// Inspect and extract metadata from a Cardano address
pub async fn inspect_address(view: &str) -> Result<AddressInspectorResponse, CexplorerError> {
    let endpoint = format!("/address/extract?view={}", view);
    fetch::<AddressInspectorResponse>(&endpoint).await
}
