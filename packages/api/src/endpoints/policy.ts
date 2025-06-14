import type { PolicyDetailResponse, PolicyOwnerResponse, PolicyStatsResponse } from "@/types/policyTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches detailed information about a specific policy by its ID.
 *
 * @param {Object} params - The parameters object.
 * @param {string} params.id - The unique identifier of the policy to fetch.
 *
 * @returns {Promise<PolicyDetailResponse>} A promise that resolves with the policy detail data.
 *
 * @throws Will throw an error if the network request fails.
 */
export const getPolicyDetail = async ({ id }: { id: string }) => {
  const url = `/policy/detail`;
  const options = {
    params: { id },
  };

  return handleFetch<PolicyDetailResponse>(url, undefined, options);
};

/**
 * Fetches statistical data for a specific policy by its ID.
 *
 * @param {Object} params - The parameters object.
 * @param {string} params.id - The unique identifier of the policy to retrieve statistics for.
 *
 * @returns {Promise<PolicyStatsResponse>} A promise that resolves with the policy statistics data.
 *
 * @throws Will throw an error if the network request fails.
 */
export const getPolicyStats = async ({ id }: { id: string }) => {
  const url = `/policy/stat`;
  const options = {
    params: { id },
  };

  return handleFetch<PolicyStatsResponse>(url, undefined, options);
};

/**
 * Fetches a paginated list of owners for a given policy.
 *
 * @param {Object} params - The parameters object.
 * @param {number} [params.limit=20] - The maximum number of results to return.
 * @param {number} [params.offset=0] - The number of results to skip.
 * @param {string} params.id - The unique identifier of the policy.
 *
 * @returns {Promise<PolicyOwnerResponse>} A promise that resolves with the list of policy owners.
 *
 * @throws Will throw an error if the network request fails.
 */
export const getPolicyOwner = async ({ limit = 20, offset = 0, id }: { limit?: number; offset?: number; id: string }) => {
  const url = `/policy/owner`;
  const options = {
    params: { limit, offset, id },
  };

  return handleFetch<PolicyOwnerResponse>(url, offset, options);
};
