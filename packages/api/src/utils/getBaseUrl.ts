import type { Network } from "@/types/configTypes";

/**
 * Returns the base API URL for the given Cexplorer network.
 *
 * @example
 * ```ts
 * const url = getBaseUrl("preprod-stage");
 * // → "https://api-preprod-stage.cexplorer.io/v1"
 * ```
 *
 * @param network - The network environment to use ("mainnet-stage", "preprod-stage", or "preview-stage")
 * @returns The base URL string for the specified network
 * @throws If an unknown network value is passed
 */
export const getBaseUrl = (network: Network): string => {
  switch (network) {
    case "mainnet-stage":
      return "https://api-mainnet-stage.cexplorer.io/v1";
    case "preprod-stage":
      return "https://api-preprod-stage.cexplorer.io/v1";
    case "preview-stage":
      return "https://api-preview-stage.cexplorer.io/v1";
    default:
      throw new Error("Unknown network");
  }
};
