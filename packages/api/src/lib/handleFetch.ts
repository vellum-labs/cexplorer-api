import type { Network } from "@/types/config";
import { getUrl } from "@/utils/getUrl";
import type { StringifiableRecord } from "query-string";

interface FetchError extends Error {
  status?: number;
}

interface FetchOptions extends RequestInit {
  params?: StringifiableRecord;
  timeout?: number;
  retryCount?: number;
  headers?: HeadersInit;
  body?: BodyInit;
}

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

export const handleFetch = async <T>(
  url: string,
  network: Network,
  prevOffset?: number,
  options?: FetchOptions
): Promise<T & { prevOffset: number | undefined }> => {
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
