import type {
  AssetDetailResponse,
  AssetListResponse,
  AssetMetadataResponse,
  AssetMintResponse,
  AssetOwnersNftResponse,
  AssetStatsResponse,
} from "@/types/assetsTypes";

import { handleFetch } from "@/lib/handleFetch";

interface AssetListProps {
  limit?: number;
  offset?: number;
  sort?: "asc" | "desc";
  order?: "collection_quantity" | "native" | "mint";
  policy?: string;
  name?: string;
  filter?: "nft" | "token";
  watchlist?: "1" | undefined;
  token?: string;
}

/**
 * Fetches a paginated list of blockchain assets from the Cexplorer API.
 *
 * Supports filtering, sorting, and watchlist-only results. Returns a list of assets
 * along with pagination metadata.
 *
 * @param {AssetListProps} params - Query parameters for asset listing.
 * @param {number} [params.limit=20] - Number of assets to return.
 * @param {number} [params.offset=0] - Offset for pagination.
 * @param {string} [params.filter] - General search filter.
 * @param {string} [params.name] - Asset name filter.
 * @param {string} [params.order] - Sort order (e.g. ASC, DESC).
 * @param {string} [params.policy] - Policy ID to filter by.
 * @param {string} [params.sort] - Field to sort by.
 * @param {"1"} [params.watchlist] - If set to "1", returns only watchlist items.
 * @param {string} [params.token] - Optional user token for authenticated requests.
 *
 * @example
 * ```ts
 * const result = await getAssetList({
 *   limit: 10,
 *   offset: 0,
 *   filter: "token",
 *   watchlist: "1",
 *   token: "user-token-value"
 * });
 * ```
 *
 * @returns {Promise<AssetListResponse>} A promise resolving to the asset list response.
 * @throws If the request fails or parameters are invalid.
 */
export const getAssetList = async ({ limit = 20, offset = 0, filter, name, order, policy, sort, watchlist, token }: AssetListProps) => {
  const url = "/asset/list";
  const options = {
    params: {
      limit,
      offset,
      filter,
      name,
      order,
      policy,
      sort,
      watchlist_only: watchlist,
    },
    headers: {
      usertoken: token || "",
    },
  };

  return handleFetch<AssetListResponse>(url, offset, options);
};

/**
 * Fetches detailed information about a specific blockchain asset by its fingerprint.
 *
 * This includes metadata such as name, policy ID, supply, and other related details.
 *
 * @param {string} fingerprint – The unique asset fingerprint identifier (e.g. "asset1...").
 *
 * @example
 * ```ts
 * const asset = await getAssetDetail("asset1qrl...");
 * ```
 *
 * @returns {Promise<AssetDetailResponse>} A promise resolving to the asset detail response.
 * @throws If the request fails or the fingerprint is invalid.
 */
export const getAssetDetail = async (fingerprint: string) => {
  const url = `/asset/detail`;
  const options = {
    params: {
      fingerprint,
    },
  };

  return handleFetch<AssetDetailResponse>(url, undefined, options);
};

/**
 * Fetches the list of owners for a given blockchain asset by its asset name.
 *
 * Returns a paginated list of addresses that currently hold the specified asset.
 *
 * @param {string} assetname – The name (fingerprint or ID) of the asset.
 * @param {number} offset – The number of items to skip (for pagination).
 * @param {number} limit – The maximum number of items to return.
 *
 * @example
 * ```ts
 * const owners = await getAssetOwners("asset1qrl...", 0, 20);
 * ```
 *
 * @returns {Promise<AssetOwnersNftResponse>} A promise resolving to the list of asset owners.
 * @throws If the request fails or the asset name is invalid.
 */
export const getAssetOwners = async (assetname: string, offset: number, limit: number) => {
  const url = `/asset/owner`;
  const options = {
    params: {
      assetname,
      offset,
      limit,
    },
  };

  return handleFetch<AssetOwnersNftResponse>(url, undefined, options);
};

/**
 * Fetches the historical list of owners for a specific NFT asset.
 *
 * This endpoint provides ownership history (not just current holders) for the specified NFT asset.
 * Useful for tracing transfer and ownership changes over time.
 *
 * @param {string} assetname – The name or identifier of the NFT asset.
 * @param {number} offset – The number of records to skip (for pagination).
 * @param {number} limit – The maximum number of records to return.
 *
 * @example
 * ```ts
 * const history = await getNftAssetOwners("asset1qrl...", 0, 10);
 * ```
 *
 * @returns {Promise<AssetOwnersNftResponse>} A promise resolving to the historical ownership data.
 * @throws If the request fails or the asset name is invalid.
 */
export const getNftAssetOwners = async (assetname: string, offset: number, limit: number) => {
  const url = `/asset/owner_history`;
  const options = {
    params: {
      assetname,
      offset,
      limit,
    },
  };

  return handleFetch<AssetOwnersNftResponse>(url, undefined, options);
};

/**
 * Fetches metadata for a specific asset.
 *
 * This endpoint returns additional descriptive information about the asset,
 * such as its name, ticker, logo, and other on-chain metadata fields.
 *
 * @param {string | undefined} assetname – The unique identifier of the asset (e.g., fingerprint or policy+name).
 *
 * @example
 * ```ts
 * const metadata = await getAssetMetaData("asset1qrl...");
 * ```
 *
 * @returns {Promise<AssetMetadataResponse>} A promise resolving to the asset's metadata.
 * @throws If the asset name is missing or the request fails.
 */
export const getAssetMetaData = async (assetname: string | undefined) => {
  const url = `/asset/metadata`;
  const options = {
    params: {
      assetname,
    },
  };

  return handleFetch<AssetMetadataResponse>(url, undefined, options);
};

/**
 * Fetches minting information for a specific asset.
 *
 * This function queries the minting history of the given asset, optionally filtered by policy ID.
 * It provides information about when and how the asset was minted on the blockchain.
 *
 * @param {string | undefined} assetname – The unique identifier of the asset (typically fingerprint or policy+name).
 * @param {string} [id] – Optional policy ID to narrow down the minting context.
 *
 * @example
 * ```ts
 * const mintData = await getAssetMint("asset1qrl...", "policy123");
 * ```
 *
 * @returns {Promise<AssetMintResponse>} A promise resolving to the minting details of the asset.
 * @throws If the asset name is missing or the request fails.
 */
export const getAssetMint = async (assetname: string | undefined, id?: string) => {
  const url = `/policy/mint`;
  const options = {
    params: {
      assetname,
      id,
    },
  };

  return handleFetch<AssetMintResponse>(url, undefined, options);
};

/**
 * Fetches statistical information for a specific asset.
 *
 * This function retrieves general statistics about the given asset, such as total supply,
 * number of holders, and transaction history. The asset can be identified either by name or fingerprint.
 *
 * @param {string} [assetname] – Optional asset name (policy + name) to query.
 * @param {string} [fingerprint] – Optional CIP-14 fingerprint to identify the asset.
 *
 * @example
 * ```ts
 * const stats = await getAssetStats("asset1qrl...");
 * ```
 *
 * @returns {Promise<AssetStatsResponse>} A promise resolving to the statistical data of the asset.
 * @throws If neither assetname nor fingerprint is provided or if the request fails.
 */
export const getAssetStats = async (assetname?: string, fingerprint?: string) => {
  const url = `/asset/stat`;
  const options = {
    params: {
      assetname,
      fingerprint,
    },
  };

  return handleFetch<AssetStatsResponse>(url, undefined, options);
};

/**
 * Fetches exchange graph data for a specific asset from an external charting API.
 *
 * This function sends a POST request to an external service (`charts.dhapi.io`) to retrieve
 * historical price or trading volume data for a given asset over a specified time period.
 *
 * @param {string} assetname – The name (policy ID + asset name) of the asset to query.
 * @param {string} period – The time interval for the chart (e.g., "1min", "5min", "1hour", "1day").
 *
 * @example
 * ```ts
 * const response = await getAssetExchangesGraph("asset1qrl...", "1hour");
 * const data = await response.json();
 * ```
 *
 * @returns {Promise<Response>} A fetch Response object containing the exchange graph data.
 * @throws Will throw if the fetch request fails.
 */
export const getAssetExchangesGraph = (assetname: string, period: string) => {
  const url = "https://charts.dhapi.io/charts";

  const now = Math.floor(Date.now() / 1000);
  const durationMap: Record<string, number> = {
    "1min": 60 * 60,
    "5min": 60 * 60 * 6,
    "15min": 60 * 60 * 12,
    "30min": 60 * 60 * 24,
    "1hour": 60 * 60 * 48,
    "4hour": 60 * 60 * 24 * 7,
    "1day": 60 * 60 * 24 * 30,
  };

  const from = now - (durationMap[period] ?? 60 * 60 * 24);
  const to = now;

  return fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify({
      tokenIn: "",
      tokenOut: assetname,
      period,
      from,
      to,
    }),
  });
};
