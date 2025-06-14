import type { Network } from "@/types/configTypes";
import type { StringifiableRecord } from "query-string";

import queryString from "query-string";
import { getBaseUrl } from "./getBaseUrl";

export const getUrl = (path: string, network: Network, params?: StringifiableRecord) =>
  queryString.stringifyUrl({
    url: `${getBaseUrl(network)}${path}`,
    query: params,
  });
