import { Network } from "./env.js";

export interface CexplorerConfig {
  network: Network;
}

let config: CexplorerConfig | null = null;

export function initCexplorer(userConfig: CexplorerConfig) {
  config = userConfig;
}

export function getConfigOrThrow(): Required<CexplorerConfig> {
  if (!config) {
    throw new Error("Cexplorer SDK not initialized. Call initCexplorer({ network }) first.");
  }
  return config;
}
