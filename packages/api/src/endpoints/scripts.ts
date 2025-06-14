import type { ScriptDetailRedeemerResponse, ScriptDetailResponse, ScriptListResponse } from "@/types/scriptTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches detailed information about a specific script by its hash.
 *
 * @param {Object} params - The parameters for the request.
 * @param {string} params.hash - The hash identifier of the script.
 *
 * @returns {Promise<ScriptDetailResponse>} A promise resolving to the detailed script information.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getScriptDetail = async ({ hash }: { hash: string }) => {
  const url = `/script/detail`;
  const options = {
    params: { hash },
  };

  return handleFetch<ScriptDetailResponse>(url, undefined, options);
};

/**
 * Fetches a paginated list of redeemers for a specific script identified by hash.
 *
 * @param {Object} params - Parameters for the request.
 * @param {string | undefined} params.hash - The hash of the script.
 * @param {number} [params.limit=20] - Maximum number of redeemers to fetch.
 * @param {number} [params.offset=0] - Pagination offset.
 *
 * @returns {Promise<ScriptDetailRedeemerResponse>} A promise resolving to the list of redeemers.
 *
 * @throws Will throw an error if the request fails or response is invalid.
 */
export const getScriptDetailRedeemer = async ({ limit = 20, offset = 0, hash }: { limit?: number; offset?: number; hash: string | undefined }) => {
  const url = `/script/detail_redeemer`;
  const options = {
    params: { hash, limit, offset },
  };

  return handleFetch<ScriptDetailRedeemerResponse>(url, offset, options);
};

/**
 * Fetches a paginated list of scripts with optional filtering and sorting.
 *
 * @param {Object} params - Parameters for the request.
 * @param {number} [params.limit=20] - Maximum number of scripts to fetch.
 * @param {number} [params.offset=0] - Pagination offset.
 * @param {string} [params.hash] - Optional hash to filter scripts.
 * @param {"tx" | "redeemer.count" | "tx_payment_cred.out.sum"} [params.order] - Field to order the results by.
 *
 * @returns {Promise<ScriptListResponse>} A promise resolving to the list of scripts.
 *
 * @throws Will throw an error if the fetch request fails or returns invalid data.
 */
export const getScriptList = async ({
  limit = 20,
  offset = 0,
  hash,
  order,
}: {
  limit?: number;
  offset?: number;
  hash?: string;
  order?: "tx" | "redeemer.count" | "tx_payment_cred.out.sum";
}) => {
  const url = `/script/list`;
  const options = {
    params: { limit, offset, hash, order },
  };

  return handleFetch<ScriptListResponse>(url, offset, options);
};
