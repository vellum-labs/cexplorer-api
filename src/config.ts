import type { CexplorerConfig } from "./types/configTypes";

// Internal variable to store the current configuration
let currentConfig: CexplorerConfig | null = null;

/**
 * Initializes the Cexplorer SDK with the given configuration.
 *
 * Both `network` and `apiKey` fields are required, otherwise an error will be thrown.
 * Calling this multiple times will merge the new config with the existing one.
 *
 * @example
 * ```ts
 * initApi({
 *   network: "mainnet-stage",
 *   apiKey: "your-api-key-here"
 * });
 * ```
 *
 * @param config - Configuration object containing network and apiKey
 * @throws If the `network` or `apiKey` fields are missing or invalid
 */
export const initApi = (config: CexplorerConfig) => {
  if (!config.network) {
    throw new Error('Missing required "network" in config. Please provide one of: "mainnet-stage", "preprod-stage", "preview-stage"');
  }

  if (!config.apiKey) {
    throw new Error('Missing required "apiKey" in config. Get your API key from https://beta.cexplorer.io -> Profile -> API');
  }

  if (typeof config.apiKey !== 'string' || config.apiKey.trim().length === 0) {
    throw new Error('Invalid API key format. API key must be a non-empty string. Get your API key from https://beta.cexplorer.io -> Profile -> API');
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
    throw new Error('Cexplorer SDK not initialized. Call initApi({ network: "...", apiKey: "..." }) first.');
  }
  return currentConfig;
};
