import type { CompareWalletsResponse } from "@/types/walletTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches detailed comparison information for wallets from the articles endpoint.
 *
 * This request fetches a specific article page with the URL "wallets" in English.
 *
 * @returns {Promise<CompareWalletsResponse>} A promise resolving to the wallet comparison data.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const compareWallets = async () => {
  const url = `/article/detail`;
  const options = {
    params: {
      lng: "en",
      type: "page",
      url: "wallets",
    },
  };

  return handleFetch<CompareWalletsResponse>(url, undefined, options);
};
