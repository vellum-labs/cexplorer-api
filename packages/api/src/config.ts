import type { CexplorerConfig } from "./types/configTypes";

// Internal variable to store the current configuration
let currentConfig: CexplorerConfig | null = null;

/**
 * Initializes the Cexplorer SDK with the given configuration.
 *
 * Must include a `network` field, otherwise an error will be thrown.
 * Calling this multiple times will merge the new config with the existing one.
 *
 * @example
 * ```ts
 * initApi({ network: "preprod-stage" });
 * ```
 *
 * @param config - Partial configuration object containing at least the network
 * @throws If the `network` field is missing
 */
export const initApi = (config: Partial<CexplorerConfig>) => {
  if (!config.network) {
    throw new Error('Missing required "network" in config.');
  }

  currentConfig = {
    ...currentConfig,
    ...config,
  };
};

/**
 * Retrieves the current configuration.
 *
 * This function should be used by any module that needs access to the active network.
 *
 * @returns The current configuration object
 * @throws If the SDK has not been initialized via `initApi`
 */
export const getCexplorerConfig = (): CexplorerConfig => {
  if (!currentConfig) {
    throw new Error('Cexplorer SDK not initialized. Call initApi({ network: "..." }) first.');
  }
  return currentConfig;
};
