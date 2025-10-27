import type { BlockDetailParams, BlockDetailResponse, BlocksListResponse } from "@/types/blockTypes";

import { handleFetch } from "@/lib/handleFetch";

type BlockListProps = {
  limit?: number;
  offset?: number;
  pool_id?: string;
  epoch_no?: number;
  hash?: string;
  slot_no?: number;
  block_no?: number;
};

/**
 * Fetches a list of blocks from the Cexplorer API based on provided query parameters.
 *
 * This function retrieves the current configuration via `getCexplorerConfig`, ensures
 * that a network is set, and sends a request to the `/block/list` endpoint with the
 * given filters.
 *
 * @param {BlockListProps} params - Object containing optional query parameters:
 *  - `limit` (default 20) – number of blocks to fetch
 *  - `offset` (default 0) – pagination offset
 *  - `pool_id` – pool ID to filter blocks
 *  - `epoch_no` – epoch number to filter
 *  - `hash` – block hash to filter
 *  - `slot_no` – slot number to filter
 *  - `block_no` – block number to filter
 *
 * @example
 * ```ts
 * const data = await getBlockList({ limit: 10, offset: 0 });
 * ```
 *
 * @returns {Promise<BlockListResponse>} A promise resolving to a list of blocks
 * @throws {Error} If the network is not configured or request fails
 */
export const getBlockList = async ({ limit = 20, offset = 0, pool_id, epoch_no, hash, slot_no, block_no }: BlockListProps) => {
  const url = "/block/list";
  const options = {
    params: { limit, offset, pool_id, epoch_no, hash, slot_no, block_no },
  };

  return handleFetch<BlocksListResponse>(url, offset, options);
};

/**
 * Fetches detailed information about a specific block from the Cexplorer API.
 *
 * This function retrieves the current configuration using `getCexplorerConfig`,
 * validates that a network is defined, and then fetches the block detail by its hash.
 *
 * @example
 * ```ts
 * const detail = await getBlockDetail({ hash: "abc123..." });
 * ```
 *
 * @param {BlockDetailArgs} args - The arguments containing the block hash.
 * @param {string} args.hash - The hash of the block to fetch details for.
 * @returns {Promise<BlockDetailResponse>} The detailed data of the block.
 * @throws If the configuration is missing or the network is not defined.
 */
export const getBlockDetail = async ({ hash }: BlockDetailParams) => {
  const url = `/block/detail?hash=${hash}`;

  return handleFetch<BlockDetailResponse>(url);
};
