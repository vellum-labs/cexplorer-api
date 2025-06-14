import type { EpochDetailParamResponse, EpochDetailStatsResponse, EpochListResponse } from "@/types/epochTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches the list of epochs from the server.
 *
 * This endpoint returns a list of available epochs in the blockchain,
 * typically including epoch numbers and their metadata.
 *
 * @returns {Promise<EpochListResponse>} A promise resolving to the epoch list data.
 *
 * @throws Will throw an error if the request fails or times out (default timeout: 60 seconds).
 */
export const getEpochList = async () => {
  const url = "/epoch/list";

  const options = {
    timeout: 60000,
  };

  return handleFetch<EpochListResponse>(url, undefined, options);
};

/**
 * Fetches detailed protocol parameters for a specific epoch.
 *
 * @param {number} no - The epoch number to fetch parameters for.
 * @returns {Promise<EpochDetailParamResponse>} A promise resolving to the epoch parameter details.
 *
 * @throws Will throw an error if the request fails or the response is invalid.
 */
export const getEpochDetailParam = async (no: number) => {
  const url = "/epoch/param";
  const options = {
    params: { no },
  };

  return handleFetch<EpochDetailParamResponse>(url, undefined, options);
};

/**
 * Fetches statistical data for a specific epoch.
 *
 * @param {number} no - The epoch number to retrieve statistics for.
 * @returns {Promise<EpochDetailStatsResponse>} A promise resolving to the epoch statistics response.
 *
 * @throws Will throw an error if the fetch request fails or the response is invalid.
 */
export const getEpochDetailStats = async (no: number) => {
  const url = "/epoch/stats";
  const options = {
    params: { no },
  };

  return handleFetch<EpochDetailStatsResponse>(url, undefined, options);
};
