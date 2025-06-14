import type {
  DelegEpochRegisteredResponse,
  PoolAwardsResponse,
  PoolBirthdaysResponse,
  PoolBlocksResponse,
  PoolDelegatorsResponse,
  PoolDelegatorStatsResponse,
  PoolDetailResponse,
  PoolRewardsResponse,
  PoolsListResponse,
  PoolUpdateResponse,
  RetiredPoolsResponse,
  StakeDrepsNotSpoResponse,
  TopMarginsWithDelegatorsResponse,
  TopMultiDelegatorsResponse,
} from "@/types/poolTypes";

import { handleFetch } from "@/lib/handleFetch";
import type { AddressDetailParams } from "./stake";

interface PoolListArgs {
  limit?: number;
  offset?: number;
  sort?: "asc" | "desc";
  order?:
    | "ranking"
    | "live_stake"
    | "active_stake"
    | "delegators"
    | "pledge"
    | "pledged"
    | "roa_lifetime"
    | "average_stake"
    | "blocks"
    | "roa_recent"
    | "blocks_epoch"
    | "blocks_total"
    | "slot_update"
    | "new"
    | "update"
    | "top_delegator"
    | "leverage";
  name?: string;
  pool_id?: string;
  token?: string;
  watchlistOnly?: "1" | undefined;
  gov_action?: string;
  is_drep?: number;
  is_not_drep?: number;
}

interface PoolRewardsArgs {
  limit?: number;
  offset?: number;
  sort?: "asc" | "desc";
  order?: string;
  name?: string;
  pool_id?: string;
}

interface PoolDetailArgs {
  pool_id: string | undefined;
  hash_raw: string | undefined;
}

interface PoolUpdateArgs {
  pool_id: string;
}

/**
 * Fetches the list of blocks produced by a specific stake pool.
 *
 * @param {Object} params - The parameters object.
 * @param {string} params.pool_id - The unique identifier of the stake pool.
 *
 * @returns {Promise<PoolBlocksResponse>} A promise that resolves with the list of blocks.
 *
 * @throws Will throw an error if the request fails.
 */
export const getPoolBlocks = async ({ pool_id }: { pool_id: string }) => {
  const url = `/pool/block`;
  const options = {
    params: { pool_id },
  };

  return handleFetch<PoolBlocksResponse>(url, undefined, options);
};

/**
 * Fetches a list of delegators for a given stake pool.
 *
 * @param {Object} params - The parameters for fetching delegators.
 * @param {number} [params.limit=20] - Number of results to return.
 * @param {number} [params.offset=0] - Offset for pagination.
 * @param {string} params.pool_id - The stake pool ID to fetch delegators for.
 * @param {"gone" | "live"} params.type - Type of delegators to fetch.
 * @param {"asc" | "desc"} [params.sort] - Sorting direction.
 * @param {"live_stake" | "slot_update"} [params.order] - Field to sort by.
 *
 * @returns {Promise<PoolDelegatorsResponse>} A promise that resolves with the list of pool delegators.
 *
 * @throws Will throw an error if the request fails.
 */
export const getPoolDelegators = async ({
  limit = 20,
  offset = 0,
  pool_id,
  type,
  order,
  sort,
}: {
  limit?: number;
  offset?: number;
  pool_id: string;
  type: "gone" | "live";
  sort: "asc" | "desc" | undefined;
  order: "live_stake" | "slot_update" | undefined;
}) => {
  const url = `/pool/delegator`;
  const options = {
    params: { limit, offset, pool_id, type, order, sort },
  };

  return handleFetch<PoolDelegatorsResponse>(url, offset, options);
};

/**
 * Fetches the reward history for a given stake pool.
 *
 * @param {PoolRewardsArgs} params - The parameters for fetching pool rewards.
 * @param {number} [params.limit=20] - Number of results to return.
 * @param {number} [params.offset=0] - Offset for pagination.
 * @param {string} [params.name] - Optional name filter for the rewards.
 * @param {string} [params.pool_id] - ID of the pool to fetch rewards for.
 *
 * @returns {Promise<PoolRewardsResponse>} A promise that resolves with the pool reward data.
 *
 * @throws Will throw an error if the request fails.
 */
export const getPoolReward = async ({ limit = 20, offset = 0, name, pool_id }: PoolRewardsArgs) => {
  const url = `/pool/reward`;
  const options = {
    params: { limit, offset, name, pool_id },
  };

  return handleFetch<PoolRewardsResponse>(url, offset, options);
};

/**
 * Fetches detailed information about a specific stake pool.
 *
 * @param {PoolDetailArgs} params - Parameters to identify the pool.
 * @param {string} [params.pool_id] - The bech32-encoded pool ID.
 * @param {string} [params.hash_raw] - The raw hash of the pool.
 *
 * @returns {Promise<PoolDetailResponse>} A promise that resolves with detailed stake pool information.
 *
 * @throws Will throw an error if the request fails.
 */
export const getPoolDetail = async ({ pool_id, hash_raw }: PoolDetailArgs) => {
  const url = `/pool/detail`;
  const options = {
    params: { hash_raw, pool_id },
  };

  return handleFetch<PoolDetailResponse>(url, undefined, options);
};

/**
 * Fetches a paginated list of stake pools with optional filters.
 *
 * @param {PoolListArgs} params - Parameters for fetching the pool list.
 * @param {number} [params.limit=20] - Number of pools to fetch.
 * @param {number} [params.offset=0] - Offset for pagination.
 * @param {string} [params.sort="desc"] - Sort direction.
 * @param {string} [params.order="ranking"] - Field to sort by.
 * @param {string} [params.name] - Filter by pool name.
 * @param {string} [params.pool_id] - Filter by pool ID.
 * @param {string} [params.token] - User token for authorization.
 * @param {boolean} [params.watchlistOnly] - If true, fetch only pools in watchlist.
 * @param {string} [params.gov_action] - Filter by governance action.
 * @param {boolean} [params.is_drep] - Filter pools that are also DReps.
 * @param {boolean} [params.is_not_drep] - Filter pools that are not DReps.
 *
 * @returns {Promise<PoolsListResponse>} A promise resolving to the list of stake pools.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getPoolsList = async ({
  limit = 20,
  offset = 0,
  sort = "desc",
  order = "ranking",
  name,
  pool_id,
  token,
  watchlistOnly,
  gov_action,
  is_drep,
  is_not_drep,
}: PoolListArgs) => {
  const url = "/pool/list";
  const options = {
    headers: { usertoken: token || "" },
    params: {
      limit,
      offset,
      sort,
      order,
      name,
      pool_id,
      watchlist_only: watchlistOnly,
      gov_action,
      is_drep,
      is_not_drep,
    },
  };

  return handleFetch<PoolsListResponse>(url, offset, options);
};

/**
 * Fetches the birthday (creation date) information for a specific stake pool.
 *
 * @param {PoolListArgs} params - Parameters for the request.
 * @param {string} params.pool_id - The ID of the pool to fetch birthday data for.
 *
 * @returns {Promise<PoolBirthdaysResponse>} A promise that resolves with the birthday data of the pool.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getPoolsBirthdays = async ({ pool_id }: PoolListArgs) => {
  const url = "/pool/birthday";
  const options = {
    params: { pool_id },
  };

  return handleFetch<PoolBirthdaysResponse>(url, undefined, options);
};

/**
 * Fetches the update history and metadata for a specific stake pool.
 *
 * @param {PoolUpdateArgs} params - Parameters for the request.
 * @param {string} params.pool_id - The ID of the stake pool to retrieve update information for.
 *
 * @returns {Promise<PoolUpdateResponse>} A promise that resolves to the stake pool's update data.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getPoolUpdate = async ({ pool_id }: PoolUpdateArgs) => {
  const url = "/pool/update";
  const options = {
    params: { pool_id },
  };

  return handleFetch<PoolUpdateResponse>(url, undefined, options);
};

/**
 * Fetches the award data for a specific stake pool.
 *
 * @param {PoolUpdateArgs} params - Parameters for the request.
 * @param {string} params.pool_id - The ID of the stake pool to retrieve award information for.
 *
 * @returns {Promise<PoolAwardsResponse>} A promise that resolves to the stake pool's award data.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getPoolAwards = async ({ pool_id }: PoolUpdateArgs) => {
  const url = "/pool/award";
  const options = {
    params: { pool_id },
  };

  return handleFetch<PoolAwardsResponse>(url, undefined, options);
};

/**
 * Retrieves statistical data about delegators of a specific stake pool.
 *
 * @param {{ pool_id: string }} params - Parameters for the request.
 * @param {string} params.pool_id - The ID of the stake pool.
 *
 * @returns {Promise<PoolDelegatorStatsResponse>} A promise resolving to the delegator statistics.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getPoolDelegatorsStats = async ({ pool_id }: { pool_id: string }) => {
  const url = "/pool/delegator_stats";
  const options = {
    params: { pool_id },
  };

  return handleFetch<PoolDelegatorStatsResponse>(url, undefined, options);
};

/**
 * Fetches a paginated list of global stake pool awards.
 *
 * @param {Object} params - Request parameters.
 * @param {number} [params.limit=20] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<PoolAwardsResponse>} A promise that resolves to the list of pool awards.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getGlobalPoolAwards = async ({ limit = 20, offset = 0 }: { limit?: number; offset?: number }) => {
  const url = "/pool/award";
  const options = {
    params: { limit, offset },
  };

  return handleFetch<PoolAwardsResponse>(url, undefined, options);
};

/**
 * Fetches the "about" information for a specific stake pool.
 *
 * @param {Object} params - The parameters for the request.
 * @param {string} params.pool_id - The unique identifier of the stake pool.
 *
 * @returns {Promise<PoolAwardsResponse>} A promise that resolves to the stake pool's about information.
 *
 * @throws Will throw an error if the request fails or the response is invalid.
 */
export const getPoolAbout = async ({ pool_id }: PoolUpdateArgs) => {
  const url = "/pool/about";
  const options = {
    params: { pool_id },
  };

  return handleFetch<PoolAwardsResponse>(url, undefined, options);
};

/**
 * Fetches analytics data for top stake pools by margin, including delegator statistics.
 *
 * @param {Object} params - Parameters for fetching top margin pools.
 * @param {string} params.type - Type of analytics to retrieve (e.g., "margin", "delegators").
 * @param {number} params.offset - Number of items to skip (for pagination).
 * @param {number} params.limit - Maximum number of records to retrieve.
 *
 * @returns {Promise<TopMarginsWithDelegatorsResponse>} A promise resolving to the top pools data with margins and delegator info.
 *
 * @throws Will throw an error if the fetch request fails or returns an invalid response.
 */
export const getTopMarginsWithDelegators = async ({ type, offset, limit }) => {
  const url = "/analytics/top_pool";
  const options = {
    params: { type, offset, limit },
  };

  return handleFetch<TopMarginsWithDelegatorsResponse>(url, offset, options);
};

/**
 * Fetches a list of retired stake pools with optional filtering and pagination.
 *
 * @param {Object} params - The parameters for the request.
 * @param {"live" | "active"} [params.type="live"] - Type of delegation context to use ("live" or "active").
 * @param {number} [params.limit=20] - Maximum number of records to fetch.
 * @param {number} [params.offset=0] - Number of records to skip (for pagination).
 * @param {"date" | "live_stake"} [params.order="date"] - Field to sort the results by.
 *
 * @returns {Promise<RetiredPoolsResponse>} A promise that resolves to the list of retired pools.
 *
 * @throws Will throw an error if the fetch fails or the response is invalid.
 */
export const getRetiredPools = async ({
  type = "live",
  limit = 20,
  offset = 0,
  order = "date",
}: AddressDetailParams & {
  type?: "live" | "active";
  order?: "date" | "live_stake";
}) => {
  const url = "/pool/retired";

  const options = {
    params: {
      type,
      limit,
      offset,
      order,
    },
  };

  return handleFetch<RetiredPoolsResponse>(url, offset, options);
};

/**
 * Fetches a list of top multi-delegators for analytics purposes.
 *
 * @param {Object} params - Parameters for the request.
 * @param {number} [params.limit=20] - Maximum number of records to retrieve.
 * @param {number} [params.offset=0] - Number of records to skip (for pagination).
 *
 * @returns {Promise<TopMultiDelegatorsResponse>} A promise that resolves to the list of top multi-delegators.
 *
 * @throws Will throw an error if the request fails or returns an unexpected response.
 */
export const getTopMultiDelegators = async ({ limit = 20, offset = 0 }: { limit?: number; offset?: number; type?: string }) => {
  const url = "/analytics/top_multi";

  const options = {
    params: {
      limit,
      offset,
    },
  };

  return handleFetch<TopMultiDelegatorsResponse>(url, offset, options);
};

/**
 * Fetches data about newly registered delegations per epoch for analytics purposes.
 *
 * @returns {Promise<DelegEpochRegisteredResponse>} A promise that resolves to the list of newly registered delegations per epoch.
 *
 * @throws Will throw an error if the request fails or the response is invalid.
 */
export const getDelegEpochRegistered = () => {
  const url = "/analytics/deleg?type=deleg_epoch_registered";

  return handleFetch<DelegEpochRegisteredResponse>(url);
};

/**
 * Fetches analytics data about stakes delegated to DReps who are not SPOs.
 *
 * @returns {Promise<StakeDrepsNotSpoResponse>} A promise resolving to the stake data for non-SPO DReps.
 *
 * @throws Will throw an error if the fetch operation fails.
 */
export const getStakeDrepsNotSpo = () => {
  const url = "/analytics/stake?type=stake_dreps_not_spo";

  return handleFetch<StakeDrepsNotSpoResponse>(url);
};
