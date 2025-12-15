use crate::client::{fetch, fetch_with_params};
use crate::error::CexplorerError;
use crate::types::governance_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GovActionProposalListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovActionProposalDetailParams {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovVoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gov_action_proposal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drep_voter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_voter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committee_voter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteeDetailParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteeMemberParams {
    pub ident: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstitutionListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DrepListVoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    pub gov_action: String,
}

/// Get governance action proposal list
pub async fn get_gov_action_proposal_list(
    limit: Option<u32>,
    offset: Option<u32>,
    state: Option<&str>,
    search: Option<&str>,
    action_type: Option<&str>,
) -> Result<GovernanceActionListResponse, CexplorerError> {
    let endpoint = "/gov/gov_action_proposal_list";
    let params = GovActionProposalListParams {
        limit,
        offset,
        state: state.map(|s| s.to_string()),
        search: search.map(|s| s.to_string()),
        r#type: action_type.map(|s| s.to_string()),
    };
    fetch_with_params::<GovernanceActionListResponse, GovActionProposalListParams>(
        endpoint,
        Some(&params),
    )
    .await
}

/// Get governance action proposal detail
pub async fn get_gov_action_proposal_detail(
    id: &str,
) -> Result<GovernanceActionDetailResponse, CexplorerError> {
    let endpoint = "/gov/gov_action_proposal_detail";
    let params = GovActionProposalDetailParams {
        id: id.to_string(),
    };
    fetch_with_params::<GovernanceActionDetailResponse, GovActionProposalDetailParams>(
        endpoint,
        Some(&params),
    )
    .await
}

/// Get governance votes
pub async fn get_gov_vote(
    limit: Option<u32>,
    offset: Option<u32>,
    gov_action_proposal: Option<&str>,
    voter_role: Option<&str>,
    order: Option<&str>,
    sort: Option<&str>,
    vote: Option<&str>,
    search: Option<&str>,
    tx: Option<&str>,
    drep_voter: Option<&str>,
    pool_voter: Option<&str>,
    committee_voter: Option<&str>,
) -> Result<GovVoteResponse, CexplorerError> {
    let endpoint = "/gov/vote";
    let params = GovVoteParams {
        limit,
        offset,
        gov_action_proposal: gov_action_proposal.map(|s| s.to_string()),
        voter_role: voter_role.map(|s| s.to_string()),
        order: order.map(|s| s.to_string()),
        sort: sort.map(|s| s.to_string()),
        vote: vote.map(|s| s.to_string()),
        search: search.map(|s| s.to_string()),
        tx: tx.map(|s| s.to_string()),
        drep_voter: drep_voter.map(|s| s.to_string()),
        pool_voter: pool_voter.map(|s| s.to_string()),
        committee_voter: committee_voter.map(|s| s.to_string()),
    };
    fetch_with_params::<GovVoteResponse, GovVoteParams>(endpoint, Some(&params)).await
}

/// Get governance votes NOT voted
pub async fn get_gov_vote_not(
    limit: Option<u32>,
    offset: Option<u32>,
    gov_action_proposal: Option<&str>,
    voter_role: Option<&str>,
    order: Option<&str>,
    sort: Option<&str>,
    search: Option<&str>,
) -> Result<GovVoteResponse, CexplorerError> {
    let endpoint = "/gov/vote_not";
    let params = GovVoteParams {
        limit,
        offset,
        gov_action_proposal: gov_action_proposal.map(|s| s.to_string()),
        voter_role: voter_role.map(|s| s.to_string()),
        order: order.map(|s| s.to_string()),
        sort: sort.map(|s| s.to_string()),
        vote: None,
        search: search.map(|s| s.to_string()),
        tx: None,
        drep_voter: None,
        pool_voter: None,
        committee_voter: None,
    };
    fetch_with_params::<GovVoteResponse, GovVoteParams>(endpoint, Some(&params)).await
}

/// Get committee list
pub async fn get_committee_list() -> Result<CommitteeListResponse, CexplorerError> {
    let endpoint = "/gov/committee_list/";
    fetch::<CommitteeListResponse>(endpoint).await
}

/// Get committee detail
pub async fn get_committee_detail(
    id: Option<u32>,
) -> Result<CommitteeDetailResponse, CexplorerError> {
    let endpoint = "/gov/committee_detail";
    let params = CommitteeDetailParams { id };
    fetch_with_params::<CommitteeDetailResponse, CommitteeDetailParams>(endpoint, Some(&params))
        .await
}

/// Get committee member detail
pub async fn get_committee_member(
    ident: &str,
) -> Result<CCMemberDetailResponse, CexplorerError> {
    let endpoint = "/gov/committee_member";
    let params = CommitteeMemberParams {
        ident: ident.to_string(),
    };
    fetch_with_params::<CCMemberDetailResponse, CommitteeMemberParams>(endpoint, Some(&params))
        .await
}

/// Get constitution list
pub async fn get_constitution_list(
    limit: Option<u32>,
) -> Result<ConstitutionListResponse, CexplorerError> {
    let endpoint = "/gov/constitution_list";
    let params = ConstitutionListParams { limit };
    fetch_with_params::<ConstitutionListResponse, ConstitutionListParams>(endpoint, Some(&params))
        .await
}

/// Get governance thresholds
pub async fn get_thresholds() -> Result<ThresholdResponse, CexplorerError> {
    let endpoint = "/gov/thresholds";
    fetch::<ThresholdResponse>(endpoint).await
}

/// Get drep list vote
pub async fn get_drep_list_vote(
    limit: Option<u32>,
    offset: Option<u32>,
    gov_action: &str,
) -> Result<DrepListVoteResponse, CexplorerError> {
    let endpoint = "/gov/drep_list_vote";
    let params = DrepListVoteParams {
        limit,
        offset,
        gov_action: gov_action.to_string(),
    };
    fetch_with_params::<DrepListVoteResponse, DrepListVoteParams>(endpoint, Some(&params)).await
}
