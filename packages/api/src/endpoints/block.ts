import { getCexplorerConfig } from "@/config";
import { getBaseUrl } from "@/utils/getBaseUrl";

export const fetchBlockList = () => {
  const { network } = getCexplorerConfig();

  if (!network) {
    throw new Error('Missing required "network" in config.');
  }

  const baseUrl = getBaseUrl(network);

  return baseUrl;
};
