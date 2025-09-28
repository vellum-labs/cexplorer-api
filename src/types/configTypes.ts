export type Network = "mainnet-stage" | "preprod-stage" | "preview-stage";

export interface CexplorerConfig {
  network: Network;
  apiKey: string;
}
