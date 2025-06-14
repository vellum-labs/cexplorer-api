import { getConfigOrThrow } from "./config.js";
import { getBaseUrl } from "./env.js";

export function test(path: string) {
  const { network } = getConfigOrThrow();
  const url = `${getBaseUrl(network)}${path}`;

  return url;
}
