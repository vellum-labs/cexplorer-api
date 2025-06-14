import type { Network } from "@/config";

export const getBaseUrl = (network: Network) => {
  switch (network) {
    case "mainnet-stage":
      return "https://api-mainnet-stage.cexplorer.io/v1";
    case "preprod-stage":
      return "https://api-preprod-stage.cexplorer.io/v1";
    case "preview-stage":
      return "https://api-preview-stage.cexplorer.io/v";
    default:
      throw new Error("Unknown network");
  }
};
