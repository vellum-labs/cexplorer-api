import { getCexplorerConfig } from "@/config";
import { handleFetch } from "@/lib/handleFetch";
import type { BlockListProps, BlockListResponse } from "@/types/block";

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
 * const data = await fetchBlockList({ limit: 10, offset: 0 });
 * console.log(data.items);
 * ```
 *
 * @returns {Promise<BlockListResponse>} A promise resolving to a list of blocks
 * @throws {Error} If the network is not configured or request fails
 */
export const fetchBlockList = async ({ limit = 20, offset = 0, pool_id, epoch_no, hash, slot_no, block_no }: BlockListProps) => {
  const { network } = getCexplorerConfig();

  if (!network) {
    throw new Error('Missing required "network" in config.');
  }

  const url = "/block/list";
  const options = {
    params: { limit, offset, pool_id, epoch_no, hash, slot_no, block_no },
  };

  return handleFetch<BlockListResponse>(url, network, offset, options);
};
