import { handleFetch } from "@/lib/handleFetch";
import type { DeFiOrderListResponse, DeFiTokenListResponse, DeFiTokenStatResponse } from "@/types/tokenTypes";

export type DeFiTokenOrder = "price_ada" | "volume" | "liquidity_ada" | undefined;

interface DeFiTokenListParams {
  limit?: number;
  offset?: number;
  order?: DeFiTokenOrder;
  assetname?: string;
  sort?: "asc" | "desc";
}

interface DeFiOrderListParams {
  limit?: number;
  offset?: number;
  address?: string;
  stake?: string;
  status?: string;
  dex?: string;
  tx?: string;
  fingerprint?: string;
  token_in?: string;
  token_out?: string;
}

/**
 * Fetches a paginated list of DeFi tokens with optional sorting and filtering.
 *
 * @param {DeFiTokenListParams} params - Parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of tokens to fetch.
 * @param {number} [params.offset=0] - Pagination offset.
 * @param {string} [params.sort="asc"] - Sort direction ("asc" or "desc").
 * @param {string} [params.order="volume"] - Field to order the results by.
 * @param {string} [params.assetname] - Optional filter by asset name.
 *
 * @returns {Promise<DeFiTokenListResponse>} A promise resolving to the list of DeFi tokens.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getDefiTokenList = async ({ limit = 10, offset = 0, sort = "asc", order = "volume", assetname }: DeFiTokenListParams) => {
  const url = `/defi/token`;
  const options = {
    params: {
      limit,
      offset,
      order,
      sort,
      assetname,
    },
  };

  return handleFetch<DeFiTokenListResponse>(url, offset, options);
};

/**
 * Fetches statistical data about DeFi tokens.
 *
 * @returns {Promise<DeFiTokenStatResponse>} A promise that resolves to DeFi token statistics.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getDefiTokenStat = async () => {
  const url = `/defi/stat`;

  return handleFetch<DeFiTokenStatResponse>(url);
};

/**
 * Fetches a paginated list of DeFi orders with optional filtering parameters.
 *
 * @param {DeFiOrderListParams} params - Parameters to filter and paginate the order list.
 * @param {number} [params.limit=10] - Maximum number of orders to fetch.
 * @param {number} [params.offset=0] - Pagination offset.
 * @param {string} [params.address] - Address involved in the order.
 * @param {string} [params.stake] - Stake identifier filter.
 * @param {string} [params.status] - Status filter for the orders.
 * @param {string} [params.dex] - Decentralized exchange identifier.
 * @param {string} [params.tx] - Transaction hash filter.
 * @param {string} [params.fingerprint] - Token fingerprint filter.
 * @param {string} [params.token_in] - Input token filter.
 * @param {string} [params.token_out] - Output token filter.
 *
 * @returns {Promise<DeFiOrderListResponse>} A promise resolving to the list of filtered DeFi orders.
 *
 * @throws Will throw an error if the fetch request fails.
 */
export const getDeFiOrder = async ({
  limit = 10,
  offset = 0,
  address,
  stake,
  status,
  dex,
  tx,
  fingerprint,
  token_in,
  token_out,
}: DeFiOrderListParams) => {
  const url = `/defi/order`;
  const options = {
    params: {
      limit,
      offset,
      address,
      stake,
      status,
      dex,
      tx,
      token: fingerprint,
      token_in,
      token_out,
    },
  };

  return handleFetch<DeFiOrderListResponse>(url, offset, options);
};
