import type { StakeDetailResponse } from "@/types/stakeTypes";

import { handleFetch } from "@/lib/handleFetch";

export interface AddressDetailParams {
  view?: string;
  limit?: number;
  offset?: number;
}

/**
 * Fetches detailed stake account information by view identifier.
 *
 * @param {AddressDetailParams} params - Parameters including the view identifier.
 * @param {string} params.view - The view identifier for the stake account.
 *
 * @returns {Promise<StakeDetailResponse>} A promise resolving to the stake account details.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getStakeDetail = async ({ view }: AddressDetailParams) => {
  const url = `/account/detail?view=${view}`;

  return handleFetch<StakeDetailResponse>(url);
};
