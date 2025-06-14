import type { TxDetailParams, TxDetailResponse, TxListParams, TxListResponse } from "@/types/txTypes";

import { handleFetch } from "@/lib/handleFetch";
import type { DrepRegistrationsResponse } from "@/types/drepTypes";
import type { PoolRegistrationsResponse } from "@/types/poolTypes";
import type { StakeRegistrationsResponse } from "@/types/stakeTypes";
import type { ContractInteractionsResponse } from "@/types/contractTypes";

/**
 * Fetches detailed information about a specific transaction by its hash.
 *
 * @param {TxDetailParams} params - Parameters containing the transaction hash.
 * @param {string} params.hash - The transaction hash to fetch details for.
 *
 * @returns {Promise<TxDetailResponse>} A promise resolving to the transaction detail data.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getTxDetail = async ({ hash }: TxDetailParams) => {
  const url = `/tx/detail?hash=${hash}`;

  return handleFetch<TxDetailResponse>(url);
};

/**
 * Fetches a paginated list of transactions with optional filters.
 *
 * @param {TxListParams} params - Parameters for fetching transaction list.
 * @param {string} [params.hash] - Filter transactions by hash.
 * @param {number} [params.limit=10] - Maximum number of transactions to fetch.
 * @param {number} [params.offset=0] - Pagination offset.
 * @param {string} [params.address] - Filter transactions by address.
 * @param {string} [params.stake] - Filter transactions by stake.
 * @param {string} [params.asset] - Filter transactions by asset.
 * @param {string} [params.script] - Filter transactions by script.
 * @param {boolean} [params.has_donation] - Filter transactions by donation presence.
 * @param {string} [params.policy] - Filter transactions by policy.
 *
 * @returns {Promise<TxListResponse>} A promise resolving to the filtered list of transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getTxList = async ({ hash, limit = 10, offset = 0, address, stake, asset, script, has_donation, policy }: TxListParams) => {
  const url = `/tx/list`;
  const options = {
    params: {
      limit,
      offset,
      hash,
      address,
      stake,
      asset,
      script,
      has_donation,
      policy,
    },
  };

  return handleFetch<TxListResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of DRep registration transactions.
 *
 * @param {TxListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<DrepRegistrationsResponse>} A promise resolving to the list of DRep registration transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getDrepRegistrations = async ({ limit = 10, offset = 0 }: TxListParams) => {
  const url = `/tx/filter`;
  const options = {
    params: {
      limit,
      offset,
      type: "drep_registrations",
    },
  };

  return handleFetch<DrepRegistrationsResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of DRep deregistration transactions.
 *
 * @param {TxListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<DrepRegistrationsResponse>} A promise resolving to the list of DRep deregistration transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getDrepDeregistrations = async ({ limit = 10, offset = 0 }: TxListParams) => {
  const url = `/tx/filter`;
  const options = {
    params: {
      limit,
      offset,
      type: "drep_deregistrations",
    },
  };

  return handleFetch<DrepRegistrationsResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of DRep update transactions.
 *
 * @param {TxListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<DrepRegistrationsResponse>} A promise resolving to the list of DRep update transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getDrepUpdates = async ({ limit = 10, offset = 0 }: TxListParams) => {
  const url = `/tx/filter`;
  const options = {
    params: {
      limit,
      offset,
      type: "drep_updates",
    },
  };

  return handleFetch<DrepRegistrationsResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of pool registration transactions.
 *
 * @param {TxListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<PoolRegistrationsResponse>} A promise resolving to the list of pool registration transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getPoolRegistrations = async ({ limit = 10, offset = 0 }: TxListParams) => {
  const url = `/tx/filter`;
  const options = {
    params: {
      limit,
      offset,
      type: "pool_registrations",
    },
  };

  return handleFetch<PoolRegistrationsResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of pool deregistration transactions.
 *
 * @param {TxListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<PoolRegistrationsResponse>} A promise resolving to the list of pool deregistration transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getPoolDeregistrations = async ({ limit = 10, offset = 0 }: TxListParams) => {
  const url = `/tx/filter`;
  const options = {
    params: {
      limit,
      offset,
      type: "pool_deregistrations",
    },
  };

  return handleFetch<PoolRegistrationsResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of stake key registration transactions.
 *
 * @param {TxListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<StakeRegistrationsResponse>} A promise resolving to the list of stake key registration transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getStakeRegistrations = async ({ limit = 10, offset = 0 }: TxListParams) => {
  const url = `/tx/filter`;
  const options = {
    params: {
      limit,
      offset,
      type: "stake_key_registrations",
    },
  };

  return handleFetch<StakeRegistrationsResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of contract interaction transactions.
 *
 * @param {TxListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Number of records to skip for pagination.
 *
 * @returns {Promise<ContractInteractionsResponse>} A promise resolving to the list of contract transactions.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getContractTransactions = async ({ limit = 10, offset = 0 }: TxListParams) => {
  const url = `/tx/filter`;
  const options = {
    params: {
      limit,
      offset,
      type: "contract_transactions",
    },
  };

  return handleFetch<ContractInteractionsResponse>(url, offset, options);
};
