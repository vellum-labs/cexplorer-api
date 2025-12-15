import type {
  GovernanceActionListResponse,
  GovernanceActionDetailResponse,
  GovVoteResponse,
  CommitteeListResponse,
  CommitteeDetailResponse,
  CCMemberDetailResponse,
  ConstitutionListResponse,
  ThresholdResponse,
  DrepListVoteResponse,
} from "@/types/governanceTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches the list of governance action proposals with optional filters.
 *
 * @param {number} [limit] - Maximum number of proposals to return.
 * @param {number} [offset] - Offset for pagination.
 * @param {string} [state] - Filter by proposal state (e.g., "All", "Active", "Ratified", "Enacted", "Expired").
 * @param {string} [search] - Search query string.
 * @param {string} [type] - Filter by action type.
 *
 * @returns {Promise<GovernanceActionListResponse>} A promise resolving to the governance action list.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getGovActionProposalList = async (
  limit?: number,
  offset?: number,
  state?: string,
  search?: string,
  type?: string,
) => {
  const url = "/gov/gov_action_proposal_list";
  const options = {
    params: {
      limit,
      offset,
      state,
      search,
      type,
    },
  };

  return handleFetch<GovernanceActionListResponse>(url, offset, options);
};

/**
 * Fetches detailed information about a specific governance action proposal.
 *
 * @param {string} id - The unique identifier of the governance action proposal.
 *
 * @returns {Promise<GovernanceActionDetailResponse>} A promise resolving to the proposal details.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getGovActionProposalDetail = async (id: string) => {
  const url = "/gov/gov_action_proposal_detail";
  const options = {
    params: {
      id,
    },
  };

  return handleFetch<GovernanceActionDetailResponse>(url, undefined, options);
};

/**
 * Fetches governance votes with optional filters.
 *
 * @param {number} [limit] - Maximum number of votes to return.
 * @param {number} [offset] - Offset for pagination.
 * @param {string} [gov_action_proposal] - Filter by governance action proposal ID.
 * @param {string} [voter_role] - Filter by voter role (e.g., "SPO", "DRep", "ConstitutionalCommittee").
 * @param {string} [order] - Sort order field (e.g., "stake", "represented_by").
 * @param {string} [sort] - Sort direction ("asc" or "desc").
 * @param {string} [vote] - Filter by vote type (e.g., "Yes", "No", "Abstain").
 * @param {string} [search] - Search query string.
 * @param {string} [tx] - Filter by transaction hash.
 * @param {string} [drep_voter] - Filter by DRep voter identifier.
 * @param {string} [pool_voter] - Filter by pool voter identifier.
 * @param {string} [committee_voter] - Filter by committee voter identifier.
 *
 * @returns {Promise<GovVoteResponse>} A promise resolving to the governance votes.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getGovVote = async (
  limit?: number,
  offset?: number,
  gov_action_proposal?: string,
  voter_role?: string,
  order?: string,
  sort?: string,
  vote?: string,
  search?: string,
  tx?: string,
  drep_voter?: string,
  pool_voter?: string,
  committee_voter?: string,
) => {
  const url = "/gov/vote";
  const options = {
    params: {
      limit,
      offset,
      gov_action_proposal,
      voter_role,
      order,
      sort,
      vote,
      search,
      tx,
      drep_voter,
      pool_voter,
      committee_voter,
    },
  };

  return handleFetch<GovVoteResponse>(url, undefined, options);
};

/**
 * Fetches voters who have NOT voted on a governance action.
 *
 * @param {number} [limit] - Maximum number of non-voters to return.
 * @param {number} [offset] - Offset for pagination.
 * @param {string} [gov_action_proposal] - Filter by governance action proposal ID.
 * @param {string} [voter_role] - Filter by voter role.
 * @param {string} [order] - Sort order field.
 * @param {string} [sort] - Sort direction ("asc" or "desc").
 * @param {string} [search] - Search query string.
 *
 * @returns {Promise<GovVoteResponse>} A promise resolving to the non-voters list.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getGovVoteNot = async (
  limit?: number,
  offset?: number,
  gov_action_proposal?: string,
  voter_role?: string,
  order?: string,
  sort?: string,
  search?: string,
) => {
  const url = "/gov/vote_not";
  const options = {
    params: {
      limit,
      offset,
      gov_action_proposal,
      voter_role,
      order,
      sort,
      search,
    },
  };

  return handleFetch<GovVoteResponse>(url, undefined, options);
};

/**
 * Fetches the list of constitutional committees.
 *
 * @returns {Promise<CommitteeListResponse>} A promise resolving to the committee list.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getCommitteeList = async () => {
  const url = "/gov/committee_list/";
  return handleFetch<CommitteeListResponse>(url, undefined, {});
};

/**
 * Fetches detailed information about a specific committee.
 *
 * @param {number} [id] - The committee ID (default: 0 for current committee).
 *
 * @returns {Promise<CommitteeDetailResponse>} A promise resolving to the committee details.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getCommitteeDetail = async (id?: number) => {
  const url = "/gov/committee_detail";
  const options = {
    params: {
      id,
    },
  };

  return handleFetch<CommitteeDetailResponse>(url, undefined, options);
};

/**
 * Fetches detailed information about a specific committee member.
 *
 * @param {string} ident - The committee member identifier (cold credential).
 *
 * @returns {Promise<CCMemberDetailResponse>} A promise resolving to the member details.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getCommitteeMember = async (ident: string) => {
  const url = "/gov/committee_member";
  const options = {
    params: {
      ident,
    },
  };

  return handleFetch<CCMemberDetailResponse>(url, undefined, options);
};

/**
 * Fetches the list of constitutions.
 *
 * @param {number} [limit] - Maximum number of constitutions to return (default: 10).
 *
 * @returns {Promise<ConstitutionListResponse>} A promise resolving to the constitution list.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getConstitutionList = async (limit?: number) => {
  const url = "/gov/constitution_list";
  const options = {
    params: {
      limit,
    },
  };

  return handleFetch<ConstitutionListResponse>(url, undefined, options);
};

/**
 * Fetches the current governance thresholds for different proposal types.
 *
 * @returns {Promise<ThresholdResponse>} A promise resolving to the thresholds data.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getThresholds = async () => {
  const url = "/gov/thresholds";
  return handleFetch<ThresholdResponse>(url, undefined, {});
};

/**
 * Fetches the list of DRep votes for a specific governance action.
 *
 * @param {number} [limit] - Maximum number of DRep votes to return.
 * @param {number} [offset] - Offset for pagination.
 * @param {string} gov_action - The governance action identifier.
 *
 * @returns {Promise<DrepListVoteResponse>} A promise resolving to the DRep vote list.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getDrepListVote = async (
  limit?: number,
  offset?: number,
  gov_action?: string,
) => {
  const url = "/gov/drep_list_vote";
  const options = {
    params: {
      limit,
      offset,
      gov_action,
    },
  };

  return handleFetch<DrepListVoteResponse>(url, undefined, options);
};
