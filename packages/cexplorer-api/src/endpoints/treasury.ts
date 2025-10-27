import type { TreasuryDonationStatsResponse } from "@/types/treasuryTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches statistics related to treasury donations.
 *
 * @returns {Promise<TreasuryDonationStatsResponse>} A promise that resolves to treasury donation statistics.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getTreasuryDonationStats = async () => {
  const url = "/analytics/treasury";

  return handleFetch<TreasuryDonationStatsResponse>(url);
};
