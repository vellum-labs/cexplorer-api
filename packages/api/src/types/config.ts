/**
 * Type representing available networks for the Cexplorer API.
 *
 * - `"mainnet-stage"` – Mainnet stage
 * - `"preprod-stage"` – Pre-production stage
 * - `"preview-stage"` – Preview stage
 */
export type Network = "mainnet-stage" | "preprod-stage" | "preview-stage";

/**
 * Configuration object for initializing the Cexplorer SDK.
 *
 * @property network - The selected network (required during initialization)
 */
export interface CexplorerConfig {
  network?: Network;
}
