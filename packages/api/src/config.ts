export type Network = "mainnet-stage" | "preprod-stage" | "preview-stage";

export interface CexplorerConfig {
  network?: Network;
}

let currentConfig: CexplorerConfig | null = null;

export const initCexplorer = (config: Partial<CexplorerConfig>) => {
  if (!config.network) {
    throw new Error('Missing required "network" in config.');
  }

  currentConfig = {
    ...currentConfig,
    ...config,
  };
};

export const getCexplorerConfig = (): CexplorerConfig => {
  if (!currentConfig) {
    throw new Error('Cexplorer SDK not initialized. Call initCexplorer({ network: "..." }) first.');
  }
  return currentConfig;
};
