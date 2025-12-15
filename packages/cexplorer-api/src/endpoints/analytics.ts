import type {
  AnalyticsAdaPotsResponse,
  AnalyticsPoolBlockResponse,
  AnalyticsRateResponse,
  AnalyticsTopAddressesReponse,
  AnalyticsTopStakingAccountsResponse,
  AveragePoolResponse,
  EpochAnalyticsResponse,
  GroupDetailResponse,
  GroupsListResponse,
  HardforkResponse,
  WealthCompositionResponse,
  GenesisAddressResponse,
} from "@/types/analyticsTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches a list of historical hard forks on the Cardano blockchain.
 *
 * This function queries the `/analytics/hardforks` endpoint of the Cexplorer API
 * and returns information about protocol version changes (hard forks) that occurred over time.
 *
 * @example
 * ```ts
 * const hardforks = await getHardforks();
 * ```
 *
 * @returns {Promise<HardforkResponse>} A promise resolving to the list of hard fork events.
 * @throws If the fetch request fails or the API returns an error.
 */
export const getHardforks = async () => {
  const url = "/analytics/hardforks";

  return handleFetch<HardforkResponse>(url);
};

/**
 * Fetches epoch-level analytics data from the Cexplorer API.
 *
 * This function queries the `/analytics/epoch` endpoint with predefined display parameters
 * to retrieve aggregated metrics such as total fees, transaction counts, average fees,
 * block versions, and output address statistics.
 *
 * @example
 * ```ts
 * const analytics = await getEpochAnalytics();
 * ```
 *
 * @returns {Promise<EpochAnalyticsResponse>} A promise resolving to the epoch analytics data.
 * @throws If the fetch request fails or the API returns an error.
 */
export const getEpochAnalytics = async () => {
  const url =
    "/analytics/epoch?display=sum_fee,count_tx,avg_tx_fee,block_version,tx_composition,max_block_tx_count,count_block,count_tx_out,avg_block_size,max_block_size,count_tx_out_address,count_tx_out_stake,count_tx_out_address_not_yesterday,count_tx_out_stake_not_yesterday";

  return handleFetch<EpochAnalyticsResponse>(url);
};

/**
 * Fetches current analytics rate data from the Cexplorer API.
 *
 * This function queries the `/analytics/rate` endpoint with predefined display parameters
 * to retrieve real-time metrics such as total fees, transaction and block counts,
 * pool statistics, output addresses, and block sizes.
 *
 * @example
 * ```ts
 * const rate = await getAnalyticsRate();
 * ```
 *
 * @returns {Promise<AnalyticsRateResponse>} A promise resolving to the analytics rate data.
 * @throws If the fetch request fails or the API returns an error.
 */
export const getAnalyticsRate = async () => {
  const url =
    "/analytics/rate?display=sum_fee,count_tx,avg_tx_fee,block_version,tx_composition,max_block_tx_count,count_tx_out,count_block,avg_block_size,max_block_size,count_tx_out_address,count_tx_out_stake,count_tx_out_address_not_yesterday,count_tx_out_stake_not_yesterday,count_pool_relay_uniq,count_pool";

  return handleFetch<AnalyticsRateResponse>(url);
};

/**
 * Fetches analytics data for pool blocks in a specific epoch from the Cexplorer API.
 *
 * This function queries the `/analytics/pool_block` endpoint to retrieve information
 * about block production by stake pools during the specified epoch.
 *
 * @example
 * ```ts
 * const data = await getAnalyticsPoolBlock({ epoch_no: 456 });
 * ```
 *
 * @param {Object} params - Parameters for the request.
 * @param {number} params.epoch_no - The epoch number to fetch data for.
 *
 * @returns {Promise<AnalyticsPoolBlockResponse>} A promise resolving to the analytics data for the specified epoch.
 * @throws If the fetch request fails or the API returns an error.
 */
export const getAnalyticsPoolBlock = async ({ epoch_no }: { epoch_no: number }) => {
  const url = "/analytics/pool_block";
  const options = {
    params: {
      epoch_no,
    },
  };

  return handleFetch<AnalyticsPoolBlockResponse>(url, undefined, options);
};

/**
 * Fetches a list of top staking accounts from the Cexplorer analytics API.
 *
 * This endpoint provides analytics data on the most active staking accounts,
 * optionally filtered by DRep or pool participation.
 *
 * @example
 * ```ts
 * const data = await getAnalyticsStakingAccounts({ limit: 10, drepOnly: 1 });
 * ```
 *
 * @param {Object} params - Parameters for the request.
 * @param {number} [params.limit=20] - Maximum number of accounts to return.
 * @param {number} [params.offset=0] - Offset for pagination.
 * @param {1 | 2} [params.drepOnly] - Filter by DRep status (1 or 2).
 * @param {1 | 2} [params.poolOnly] - Filter by Pool status (1 or 2).
 *
 * @returns {Promise<AnalyticsTopStakingAccountsResponse>} A promise resolving to a list of top staking accounts.
 * @throws If the fetch request fails or the API returns an error.
 */
export const getAnalyticsStakingAccounts = async ({
  limit = 20,
  offset = 0,
  drepOnly,
  poolOnly,
}: {
  limit?: number;
  offset?: number;
  drepOnly?: 2 | 1;
  poolOnly?: 2 | 1;
}) => {
  const url = `/analytics/top_account`;
  const options = {
    params: { limit, offset, drep_only: drepOnly, pool_only: poolOnly },
  };

  return handleFetch<AnalyticsTopStakingAccountsResponse>(url, offset, options);
};

/**
 * Fetches a list of top addresses from the Cexplorer analytics API.
 *
 * This function retrieves analytical data about addresses with the highest activity,
 * optionally filtered by DRep or pool participation.
 *
 * @example
 * ```ts
 * const data = await getAnalyticsTopAddresses({ limit: 10, poolOnly: 1 });
 * ```
 *
 * @param {Object} params - Parameters for the request.
 * @param {number} [params.limit=20] - The number of results to return.
 * @param {number} [params.offset=0] - The number of results to skip (for pagination).
 * @param {1 | 2} [params.drepOnly] - Filter results to include only DRep addresses.
 * @param {1 | 2} [params.poolOnly] - Filter results to include only Pool addresses.
 *
 * @returns {Promise<AnalyticsTopAddressesReponse>} A promise resolving to a list of top addresses with analytics data.
 * @throws If the fetch request fails or the API returns an error response.
 */
export const getAnalyticsTopAddresses = async ({
  limit = 20,
  offset = 0,
  drepOnly,
  poolOnly,
}: {
  limit?: number;
  offset?: number;
  drepOnly?: 2 | 1;
  poolOnly?: 2 | 1;
}) => {
  const url = `/analytics/top_address`;
  const options = {
    params: { limit, offset, drep_only: drepOnly, pool_only: poolOnly },
  };

  return handleFetch<AnalyticsTopAddressesReponse>(url, offset, options);
};

/**
 * Fetches data about the overall wealth composition from the Cexplorer analytics API.
 *
 * This endpoint provides insights into how the total ADA is distributed across accounts
 * in terms of ranges or categories of holdings.
 *
 * @example
 * ```ts
 * const data = await getWealthComposition();
 * ```
 *
 * @returns {Promise<WealthCompositionResponse>} A promise resolving to the wealth composition data.
 * @throws If the fetch request fails or the API returns an error response.
 */
export const getWealthComposition = async () => {
  const url = "/analytics/wealth";

  return handleFetch<WealthCompositionResponse>(url);
};

/**
 * Fetches data about the current ADA pots from the Cexplorer analytics API.
 *
 * This includes information about various ADA reserves and treasury balances such as
 * treasury, reserves, rewards, deposits, and fees.
 *
 * @example
 * ```ts
 * const adaPots = await getAdaPots();
 * ```
 *
 * @returns {Promise<AnalyticsAdaPotsResponse>} A promise resolving to the current ADA pots data.
 * @throws If the fetch request fails or returns an error.
 */
export const getAdaPots = async () => {
  const url = "/analytics/ada_pot";

  return handleFetch<AnalyticsAdaPotsResponse>(url);
};

/**
 * Fetches a list of analytics groups from the Cexplorer API.
 *
 * These groups may represent categorized or aggregated data segments used
 * in various analytical dashboards or reports.
 *
 * @example
 * ```ts
 * const groups = await getGroupList();
 * ```
 *
 * @returns {Promise<GroupsListResponse>} A promise resolving to the list of analytics groups.
 * @throws If the fetch request fails or returns an error.
 */
export const getGroupList = async () => {
  const url = "/analytics/group_list";

  return handleFetch<GroupsListResponse>(url);
};

/**
 * Fetches detailed information about a specific analytics group by its ID.
 *
 * This function queries the Cexplorer API for group-specific data, which may include
 * metrics, members, or other analytical breakdowns related to the group.
 *
 * @param {string} id - The unique identifier of the analytics group.
 *
 * @example
 * ```ts
 * const detail = await getGroupDetail("group_123");
 * ```
 *
 * @returns {Promise<GroupDetailResponse>} A promise resolving to the details of the specified group.
 * @throws If the fetch request fails or the group ID is invalid.
 */

export const getGroupDetail = async (id: string) => {
  const url = `/analytics/group_detail?id=${id}`;

  return handleFetch<GroupDetailResponse>(url);
};

/**
 * Fetches average metrics related to staking pools from the Cexplorer API.
 *
 * Specifically retrieves data for the average number per pool (`avg_num_per_pool`).
 *
 * @example
 * ```ts
 * const average = await getAveragePool();
 * ```
 *
 * @returns {Promise<AveragePoolResponse>} A promise resolving to the average pool metrics.
 * @throws If the fetch request fails or returns an error response.
 */

export const getAveragePool = async () => {
  const url = "/analytics/avg_pool?type=avg_num_per_pool";

  return handleFetch<AveragePoolResponse>(url);
};

/**
 * Fetches genesis address analytics from the Cexplorer API.
 *
 * This endpoint provides information about genesis addresses on the Cardano blockchain.
 *
 * @example
 * ```ts
 * const genesisData = await getGenesisAddr();
 * ```
 *
 * @returns {Promise<GenesisAddrResponse>} A promise resolving to the genesis address data.
 * @throws If the fetch request fails or returns an error response.
 */
export const getGenesisAddr = async () => {
  const url = "/analytics/genesis_addr";

  return handleFetch<GenesisAddressResponse>(url);
};
