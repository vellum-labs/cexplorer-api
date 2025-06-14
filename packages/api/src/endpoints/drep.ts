import type {
  AverageDrepResponse,
  CombinedAverageDrepResponse,
  DelegEpochChangesResponse,
  DrepAnalyticsResponse,
  DrepDelegatorResponse,
  DrepDetailResponse,
  DrepListOrder,
  DrepListResponse,
  DrepSpoSameTimeResponse,
  DrepStatResponse,
  DrepVoteResponse,
  StakeDrepRetiredResponse,
  StakeIsSpoDrepResponse,
} from "@/types/drepTypes";

import { handleFetch } from "@/lib/handleFetch";
import type { DrepNotSpoSameTimeResponse, PoolDelegatorStatsResponse } from "@/types/poolTypes";

interface DrepListParams {
  limit?: number;
  offset?: number;
  view?: string;
  watchlistOnly?: "1" | undefined;
  token?: string;
  sort?: "asc" | "desc";
  order?: DrepListOrder;
  gov_action?: string;
  is_spo?: number;
  is_not_spo?: number;
}

interface DrepVoteParams {
  voter_role: "DRep" | "SPO";
  limit?: number;
  offset?: number;
}

interface DrepDelegatorParams {
  view: string;
  limit?: number;
  offset?: number;
  order: "live_stake" | "slot_update";
  filter: "migrations" | "live";
}

/**
 * Fetches DRep (Delegated Representative) statistics from the governance API.
 *
 * Sends a request to the `/gov/stat` endpoint to retrieve aggregated data
 * about the current state of governance representatives on the network.
 *
 * @returns {Promise<DrepStatResponse>} A promise that resolves with DRep statistics data.
 *
 * @throws Will throw an error if the fetch operation fails or returns an invalid response.
 *
 * @example
 * ```ts
 * const stats = await getDrepStat();
 * ```
 */
export const getDrepStat = async () => {
  const url = "/gov/stat";

  return handleFetch<DrepStatResponse>(url);
};

/**
 * Retrieves analytics data for DReps (Delegated Representatives) from the governance API.
 *
 * This function fetches detailed analytics about DRePs from the `/gov/drep_analytics` endpoint.
 * The data may include metrics such as voting activity, delegations, and performance.
 *
 * @returns {Promise<DrepAnalyticsResponse>} A promise that resolves with DRep analytics data.
 *
 * @throws Will throw an error if the fetch operation fails or returns an invalid response.
 *
 * @example
 * ```ts
 * const analytics = await getDrepAnalytics();
 * ```
 */
export const getDrepAnalytics = async () => {
  const url = "/gov/drep_analytics";

  return handleFetch<DrepAnalyticsResponse>(url);
};

/**
 * Fetches analytics data related to stake associated with retired DReps.
 *
 * This function sends a request to the `/analytics/stake` endpoint with the type `stake_drep_retired`
 * to retrieve aggregated information about stakes that were previously delegated to retired DRePs.
 *
 * @returns {Promise<StakeDrepRetiredResponse>} A promise resolving to the analytics data of retired DRep stakes.
 *
 * @throws Will throw an error if the fetch operation fails.
 *
 * @example
 * ```ts
 * const data = await getStakeDrepRetired();
 * ```
 */
export const getStakeDrepRetired = async () => {
  const url = "/analytics/stake?type=stake_drep_retired";

  return handleFetch<StakeDrepRetiredResponse>(url);
};

/**
 * Fetches a list of DReps (delegated representatives) with optional filtering, sorting, and pagination.
 *
 * This function sends a request to the `/gov/drep_list` endpoint using the provided parameters.
 * You can filter by watchlist status, view, governance actions, and SPO status, as well as apply sorting.
 *
 * @param {DrepListParams} params - The parameters for filtering, sorting, and pagination.
 * @param {number} [params.limit=10] - Maximum number of items to retrieve.
 * @param {number} [params.offset=0] - Offset for pagination.
 * @param {string} [params.sort] - Field to sort by.
 * @param {string} [params.order] - Sort order ("asc" or "desc").
 * @param {string} [params.view] - Stake address or view identifier.
 * @param {string} [params.watchlistOnly] - "1" to return only items in the watchlist.
 * @param {string} [params.token] - User token for authentication.
 * @param {string} [params.gov_action] - Governance action filter.
 * @param {boolean} [params.is_spo] - Filter for SPO (stake pool operator) DReps.
 * @param {boolean} [params.is_not_spo] - Filter for non-SPO DReps.
 *
 * @returns {Promise<DrepListResponse>} Resolves with the list of DReps and metadata.
 *
 * @throws Will throw an error if the network request fails or returns an invalid response.
 */
export const getDrepList = async ({
  limit = 10,
  offset = 0,
  sort,
  order,
  view,
  watchlistOnly,
  token,
  gov_action,
  is_spo,
  is_not_spo,
}: DrepListParams) => {
  const url = `/gov/drep_list`;
  const options = {
    headers: {
      usertoken: token || "",
    },
    params: {
      limit,
      sort,
      order,
      offset,
      view,
      watchlist_only: watchlistOnly,
      gov_action,
      is_spo,
      is_not_spo,
    },
  };

  return handleFetch<DrepListResponse>(url, offset, options);
};

/**
 * Fetches detailed information about a specific DRep (delegated representative) by its identifier.
 *
 * This function retrieves data from the `/gov/drep_detail` endpoint using the provided hash.
 *
 * @param {string} hash - The unique identifier (view or hash) of the DRep to retrieve.
 *
 * @returns {Promise<DrepDetailResponse>} Resolves with the detailed information about the specified DRep.
 *
 * @throws Will throw an error if the request fails or returns an invalid response.
 */
export const getDrepDetail = async (hash: string) => {
  const url = `/gov/drep_detail?view=${hash}`;

  return handleFetch<DrepDetailResponse>(url);
};

/**
 * Retrieves a list of governance votes filtered by voter role.
 *
 * This function fetches data from the `/gov/vote` endpoint, allowing pagination and filtering
 * by the type of voter (e.g. DRep, SPO, or constitutional committee).
 *
 * @param {DrepVoteParams} params - The parameters for the request.
 * @param {string} params.voter_role - The role of the voter (e.g. "drep", "spo", "cc").
 * @param {number} [params.limit=10] - The maximum number of results to return.
 * @param {number} [params.offset=0] - The offset for pagination.
 *
 * @returns {Promise<DrepVoteResponse>} Resolves with the list of governance votes matching the filter.
 *
 * @throws Will throw an error if the request fails or the response is invalid.
 */
export const getDrepVote = async ({ voter_role, limit = 10, offset = 0 }: DrepVoteParams) => {
  const url = `/gov/vote`;

  const options = {
    params: {
      voter_role,
      limit,
      offset,
    },
  };

  return handleFetch<DrepVoteResponse>(url, offset, options);
};

/**
 * Retrieves a list of delegators for a given DRep.
 *
 * This function calls the `/gov/drep_delegator` endpoint and supports pagination
 * and filtering options such as sort order and custom filters.
 *
 * @param {DrepDelegatorParams} params - The parameters for the request.
 * @param {string} params.view - The DRep view identifier (e.g., DRep hash).
 * @param {number} [params.limit=10] - The number of results to return.
 * @param {number} [params.offset=0] - The offset for pagination.
 * @param {string} [params.filter] - Optional filter for delegators (e.g., by address or other criteria).
 * @param {string} [params.order] - Optional sort order for the results (e.g., "asc" or "desc").
 *
 * @returns {Promise<DrepDelegatorResponse>} Resolves with the list of delegators for the specified DRep.
 *
 * @throws Will throw an error if the fetch fails or if the network response is invalid.
 */
export const getDrepDelegator = async ({ view, limit = 10, offset = 0, filter, order }: DrepDelegatorParams) => {
  const url = `/gov/drep_delegator`;

  const options = {
    params: {
      view,
      filter,
      order,
      limit,
      offset,
    },
  };

  return handleFetch<DrepDelegatorResponse>(url, offset, options);
};

/**
 * Retrieves statistical information about delegators of a specific DRep.
 *
 * This function sends a request to the `/gov/drep_delegator_stats` endpoint to
 * fetch aggregated data (e.g., total stake, number of delegators) for the given DRep.
 *
 * @param {Object} params - The parameters for the request.
 * @param {string} params.view - The identifier (view) of the DRep.
 *
 * @returns {Promise<PoolDelegatorStatsResponse>} Resolves with statistical data related to the DRep's delegators.
 *
 * @throws Will throw an error if the request fails or the response is invalid.
 */
export const getDrepDelegatorStats = async ({ view }: { view: string }) => {
  const url = "/gov/drep_delegator_stats";
  const options = {
    params: { view },
  };

  return handleFetch<PoolDelegatorStatsResponse>(url, undefined, options);
};

/**
 * Fetches average statistics for DReps.
 *
 * Sends a request to the `/analytics/avg_drep` endpoint with the query
 * parameter `type=avg_num_per_drep` to retrieve analytical data related
 * to the average number of delegators or stake per DRep.
 *
 * @returns {Promise<AverageDrepResponse>} Resolves with the average DRep analytics data.
 *
 * @throws Will throw an error if the fetch operation fails.
 */
export const getAverageDrep = async () => {
  const url = "/analytics/avg_drep?type=avg_num_per_drep";

  return handleFetch<AverageDrepResponse>(url);
};

/**
 * Fetches analytics data for DReps and SPOs with power at the same time.
 *
 * Sends a request to the `/analytics/drep_spo` endpoint with the query
 * parameter `type=power_drep_spo_same_time` to retrieve information about
 * DRePs and SPOs who held power concurrently.
 *
 * @returns {Promise<DrepSpoSameTimeResponse>} Resolves with the analytical data.
 *
 * @throws Will throw an error if the fetch operation fails.
 */
export const getDrepSpoSameTime = () => {
  const url = "/analytics/drep_spo?type=power_drep_spo_same_time";

  return handleFetch<DrepSpoSameTimeResponse>(url);
};

/**
 * Fetches and combines DRep average analytics and DRep-SPO overlap data.
 *
 * This function concurrently retrieves:
 * - The average number of delegators per DRep.
 * - The data on DReps and SPOs who held power at the same time.
 *
 * Returns an object containing both datasets.
 *
 * @returns {Promise<CombinedAverageDrepResponse>} An object with `averageDrep` and `drepSpoSameTime` fields.
 *
 * @throws Will throw an error if either of the underlying fetch operations fails.
 */
export const getCombinedAverageDrep = async (): Promise<CombinedAverageDrepResponse> => {
  const [avgDrepRes, drepSpoSameTimeRes] = await Promise.all([getAverageDrep(), getDrepSpoSameTime()]);

  return {
    averageDrep: avgDrepRes.data,
    drepSpoSameTime: drepSpoSameTimeRes.data,
  };
};

/**
 * Fetches analytics data about stake accounts that are both SPOs and DReps.
 *
 * This endpoint provides information on addresses that simultaneously
 * act as Stake Pool Operators (SPOs) and Delegated Representatives (DReps),
 * offering insight into their combined influence in the network.
 *
 * @returns {Promise<StakeIsSpoDrepResponse>} A promise that resolves to the combined SPO and DRep stake data.
 *
 * @throws Will throw an error if the network request fails.
 */
export const getStakeIsSpoDrep = () => {
  const url = "/analytics/stake?type=stake_is_spo_drep";

  return handleFetch<StakeIsSpoDrepResponse>(url);
};

/**
 * Fetches analytics data about DReps who are not SPOs at the same time.
 *
 * This endpoint provides insights into the power and influence of delegated representatives (DReps)
 * who do not simultaneously act as stake pool operators (SPOs).
 *
 * @returns {Promise<DrepNotSpoSameTimeResponse>} A promise resolving to data about non-SPO DReps.
 *
 * @throws Will throw an error if the network request fails.
 */
export const getDrepNotSpoSameTime = () => {
  const url = "/analytics/drep_spo?type=power_drep_not_spo";

  return handleFetch<DrepNotSpoSameTimeResponse>(url);
};

/**
 * Fetches data about delegation changes across epochs.
 *
 * Retrieves analytics describing how delegations have changed between epochs,
 * which can be useful for analyzing staking trends and user behavior over time.
 *
 * @returns {Promise<DelegEpochChangesResponse>} A promise resolving to the delegation epoch changes data.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getDelegEpochChanges = () => {
  const url = "/analytics/deleg?type=deleg_epoch_changes";

  return handleFetch<DelegEpochChangesResponse>(url);
};
