use crate::client::fetch;
use crate::error::CexplorerError;
use crate::types::treasury_types::*;

pub async fn get_treasury_donation_stats() -> Result<TreasuryDonationStatsResponse, CexplorerError> {
    let endpoint = "/analytics/treasury";
    fetch::<TreasuryDonationStatsResponse>(endpoint).await
}
