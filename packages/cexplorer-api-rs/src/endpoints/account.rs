use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::account_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountRewardsParams {
    pub view: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalParams {
    pub view: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelegationVoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

pub async fn get_account_rewards(
    view: &str,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<AccountRewardResponse, CexplorerError> {
    let endpoint = "/account/reward";
    let params = AccountRewardsParams {
        view: view.to_string(),
        limit,
        offset,
    };
    fetch_with_params::<AccountRewardResponse, AccountRewardsParams>(endpoint, Some(&params)).await
}

pub async fn check_user_delegation(
    view: Option<&str>,
) -> Result<CheckDelegationResponse, CexplorerError> {
    let endpoint = "/account/has_delegation";
    let params = ViewParams {
        view: view.map(|s| s.to_string()),
    };
    fetch_with_params::<CheckDelegationResponse, ViewParams>(endpoint, Some(&params)).await
}

pub async fn get_withdrawals(
    view: &str,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<WithdrawalsResponse, CexplorerError> {
    let endpoint = "/account/withdrawal";
    let params = WithdrawalParams {
        view: view.to_string(),
        limit,
        offset,
    };
    fetch_with_params::<WithdrawalsResponse, WithdrawalParams>(endpoint, Some(&params)).await
}

pub async fn get_delegation_vote(
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<DrepDelegationResponse, CexplorerError> {
    let endpoint = "/account/delegation_vote";
    let params = DelegationVoteParams {
        limit,
        offset,
    };
    fetch_with_params::<DrepDelegationResponse, DelegationVoteParams>(endpoint, Some(&params)).await
}
