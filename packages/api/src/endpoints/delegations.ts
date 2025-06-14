import { handleFetch } from "@/lib/handleFetch";

import type { DelegationResponse, DelegationStateResponse, DelegationToRetiredResponse } from "@/types/delegationTypes";
import type { AddressDetailParams } from "./stake";

/**
 * Fetches the current delegation state of a given staking address.
 *
 * This function calls the `/account/delegation_state` endpoint with the provided `view` parameter,
 * which typically represents the staking address.
 *
 * @param {AddressDetailParams} params – Object containing the `view` (staking address).
 * @param {string} params.view – The staking address whose delegation state is requested.
 *
 * @returns {Promise<DelegationStateResponse>} A promise that resolves to the delegation state response.
 *
 * @throws Will throw an error if the request fails or the API returns an error.
 *
 * @example
 * ```ts
 * const response = await getDelegationsState({ view: "stake1u..." });
 * console.log(response);
 * ```
 */
export const getDelegationsState = async ({ view }: AddressDetailParams) => {
  const url = `/account/delegation_state?view=${view}`;

  return handleFetch<DelegationStateResponse>(url);
};

/**
 * Fetches the list of stake delegations for a given staking address.
 *
 * This function sends a request to the `/account/delegation` endpoint with the specified parameters
 * to retrieve delegation history data for the given staking address.
 *
 * @param {AddressDetailParams} params – Parameters for the request.
 * @param {string} params.view – The staking address to query.
 * @param {number} [params.limit=20] – The number of results to return (default: 20).
 * @param {number} [params.offset=0] – The pagination offset (default: 0).
 *
 * @returns {Promise<DelegationResponse>} A promise resolving to the delegation history data.
 *
 * @throws Will throw an error if the fetch operation fails or if the response is invalid.
 *
 * @example
 * ```ts
 * const data = await getStakeDelegations({ view: "stake1...", limit: 10 });
 * console.log(data);
 * ```
 */
export const getStakeDelegations = async ({ view, limit = 20, offset = 0 }: AddressDetailParams) => {
  const url = `/account/delegation`;

  const options = {
    params: {
      view,
      limit,
      offset,
    },
  };

  return handleFetch<DelegationResponse>(url, offset, options);
};

/**
 * Fetches the list of delegations to retired pools.
 *
 * Queries the `/account/delegation_to_retired` endpoint with provided filters to retrieve
 * delegation records made to pools that have already retired.
 *
 * @param {Object} params – Parameters for the request.
 * @param {"live" | "active"} [params.type="live"] – Type of delegation to retrieve: currently live or active at the time.
 * @param {number} [params.limit=20] – Number of results to return (default: 20).
 * @param {number} [params.offset=0] – Pagination offset (default: 0).
 * @param {"date" | "live_stake"} [params.order="date"] – Sorting order for the results.
 *
 * @returns {Promise<DelegationToRetiredResponse>} A promise resolving to the delegation data for retired pools.
 *
 * @throws Will throw an error if the fetch fails or if the response is invalid.
 *
 * @example
 * ```ts
 * const data = await getDelegationsToRetired({ type: "active", order: "live_stake" });
 * console.log(data);
 * ```
 */
export const getDelegationsToRetired = async ({
  type = "live",
  limit = 20,
  offset = 0,
  order = "date",
}: AddressDetailParams & {
  type?: "live" | "active";
  order?: "date" | "live_stake";
}) => {
  const url = "/account/delegation_to_retired";

  const options = {
    params: {
      type,
      limit,
      offset,
      order,
    },
  };

  return handleFetch<DelegationToRetiredResponse>(url, undefined, options);
};
