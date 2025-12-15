import type { TxSentResponse } from "@/types/toolTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Sends delegation or donation transaction information to track campaign participation.
 *
 * This endpoint is typically used to notify the backend about successful delegation
 * or donation transactions, allowing for campaign tracking and analytics.
 *
 * @param {string} hash - The transaction hash.
 * @param {string} poolId - The pool ID or campaign identifier.
 * @param {"delegation" | "donate"} [type="delegation"] - The type of transaction (delegation or donation).
 *
 * @returns {Promise<TxSentResponse>} A promise resolving to the transaction sent confirmation.
 *
 * @throws Will throw an error if the API request fails.
 */
export const sendTxSent = async (
  hash: string,
  poolId: string,
  type: "delegation" | "donate" = "delegation",
) => {
  const url = "/tool/tx_sent";
  const options = {
    params: {
      id: hash,
      type,
      campaign: poolId,
    },
  };

  return handleFetch<TxSentResponse>(url, undefined, options);
};
