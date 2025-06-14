import type { DatumDetailResponse } from "@/types/datumTypes";

import { handleFetch } from "@/lib/handleFetch";

interface DatumDetailParams {
  hash?: string;
}

/**
 * Fetches the detail of a datum by its hash.
 *
 * This function queries the `/datum/detail` endpoint with the provided hash parameter
 * to retrieve detailed information about the datum.
 *
 * @param {DatumDetailParams} params – Object containing the datum hash.
 * @param {string} params.hash – The hash of the datum to retrieve.
 *
 * @returns {Promise<DatumDetailResponse>} A promise that resolves to the datum detail response.
 *
 * @throws Will throw an error if the network request fails or the response is invalid.
 *
 * @example
 * ```ts
 * const response = await getDatumDetail({ hash: "abcd1234" });
 * console.log(response);
 * ```
 */
export const getDatumDetail = async ({ hash }: DatumDetailParams) => {
  const url = `/datum/detail`;
  const options = {
    params: { hash },
  };

  return handleFetch<DatumDetailResponse>(url, undefined, options);
};
