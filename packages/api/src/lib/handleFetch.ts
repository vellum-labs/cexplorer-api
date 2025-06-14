import { getCexplorerConfig } from "@/config";
import { getUrl } from "@/utils/getUrl";
import type { StringifiableRecord } from "query-string";

/**
 * Interface for extended error object with optional HTTP status.
 */
interface FetchError extends Error {
  status?: number;
}

/**
 * Additional options for API fetch requests.
 */
interface FetchOptions extends RequestInit {
  params?: StringifiableRecord;
  timeout?: number;
  retryCount?: number;
  headers?: HeadersInit;
  body?: BodyInit;
}

/**
 * Fetch wrapper that enforces a timeout.
 *
 * @param url - Full request URL
 * @param timeout - Timeout in milliseconds
 * @param options - Standard fetch options
 * @returns A Promise resolving to a Response or rejecting on timeout
 * @throws Error if the request exceeds the timeout
 */
const fetchWithTimeout = (url: string, timeout: number, options?: RequestInit): Promise<Response> => {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("Request timed out")), timeout);
    fetch(url, { ...options })
      .then((response) => {
        clearTimeout(timer);
        resolve(response);
      })
      .catch((error) => {
        clearTimeout(timer);
        reject(error);
      });
  });
};

/**
 * Generic HTTP fetch handler for interacting with the Cexplorer API.
 *
 * Automatically applies base URL, query parameters, timeouts, retries,
 * and response parsing. Also injects `prevOffset` into the final result.
 *
 * @template T - Expected response type
 *
 * @param url - Relative API endpoint path (e.g. `/block/list`)
 * @param network - Selected Cexplorer network environment
 * @param prevOffset - Optional offset for pagination tracking
 * @param options - Additional fetch options including headers, params, etc.
 *
 * @returns A Promise resolving to the parsed API response merged with `prevOffset` and headers
 * @throws {Error} If the request fails or all retries are exhausted
 */
export const handleFetch = async <T>(url: string, prevOffset?: number, options?: FetchOptions): Promise<T & { prevOffset: number | undefined }> => {
  const { network } = getCexplorerConfig();

  if (!network) {
    throw new Error('Missing required "network" in config.');
  }

  const fullUrl = getUrl(url, network, options?.params);
  const timeout = options?.timeout ?? 30000;
  const retryCount = options?.retryCount ?? 2;
  const headers = options?.headers;
  const body = options?.body;
  const method = options?.method ?? "GET";

  for (let attempt = 0; attempt <= retryCount; attempt++) {
    try {
      const response = await fetchWithTimeout(fullUrl, timeout, {
        method,
        headers,
        body,
      });

      const data: T & { code?: number; msg?: string } = await response.json();

      const responseHeaders: Record<string, string> = {};
      response.headers.forEach((value, key) => {
        responseHeaders[key] = value;
      });

      if (response.status !== 200) {
        const error: FetchError = new Error("The network response failed.");
        error.status = response.status;
        return { ...data, prevOffset: prevOffset, responseHeaders };
      }

      return { ...data, prevOffset: prevOffset, responseHeaders };
    } catch (error) {
      if (attempt !== retryCount) {
        console.warn(`Attempt ${attempt + 1} failed. Retrying...`);
        continue;
      }

      if (!(error instanceof Error)) {
        throw new Error("An unknown error occurred during fetch");
      }

      console.error("Fetch error:", error.message);
      throw error;
    }
  }

  throw new Error("Failed to fetch after retries");
};
