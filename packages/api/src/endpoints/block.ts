import { getCexplorerConfig } from "@/config";
import { handleFetch } from "@/lib/handleFetch";
import type { BlockListProps, BlockListResponse } from "@/types/block";

/**
 * Prepares the base URL for fetching the block list from the Cexplorer API.
 *
 * This function retrieves the current configuration via `getCexplorerConfig`, validates
 * that a network is defined, and returns the corresponding base URL for that network.
 *
 * @example
 * ```ts
 * const url = fetchBlockList();
 * // → "https://api-preprod-stage.cexplorer.io/v1"
 * ```
 *
 * @returns The base API URL string for the current network
 * @throws If the configuration is missing or the network is not defined
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
