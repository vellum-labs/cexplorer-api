use crate::client::fetch;
use crate::error::CexplorerError;
use crate::types::stake_types::*;

pub async fn get_stake_detail(view: &str) -> Result<StakeDetailResponse, CexplorerError> {
    let endpoint = format!("/account/detail?view={}", view);
    fetch::<StakeDetailResponse>(&endpoint).await
}
