export type Network = "mainnet-stage" | "preprod-stage" | "preview-stage";

const BASE_URL: Record<Network, string> = {
  "mainnet-stage": "https://api-mainnet-stage.cexplorer.io/v1",
  "preprod-stage": "https://api-preprod-stage.cexplorer.io/v1",
  "preview-stage": "https://api-preview-stage.cexplorer.io/v1",
};

export const getBaseUrl = (net: Network) => BASE_URL[net];
