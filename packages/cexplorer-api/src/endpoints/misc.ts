import type {
  GroupType,
  MiscApiResponse,
  MiscBasicResponse,
  MiscConstResponse,
  MiscMarketResponse,
  MiscRateResponse,
  MiscSearchResponse,
  MiscValidateResponse,
  PollListResponse,
  MiscHealthResponse,
  MiscProtocolParametersResponse,
} from "@/types/miscTypes";
import type { Locales } from "@/types/storeTypes";

import { handleFetch } from "@/lib/handleFetch";

/**
 * Fetches general information about the available Cexplorer API endpoints.
 *
 * This can be used for diagnostics, documentation purposes, or to verify the structure
 * and availability of API endpoints provided by the backend.
 *
 * @returns {Promise<MiscApiResponse>} A promise resolving to the object containing metadata about the API.
 *
 * @throws Will throw an error if the request fails.
 */
export const getMiscApi = async () => {
  const url = "/misc/api";

  return handleFetch<MiscApiResponse>(url, undefined, {});
};

/**
 * Fetches basic metadata and summary statistics from the Cexplorer API.
 *
 * This may include general network information, totals, or aggregated values
 * relevant to the Cardano blockchain.
 *
 * @returns {Promise<MiscBasicResponse>} A promise resolving to a summary of basic network statistics.
 *
 * @throws Will throw an error if the request fails.
 */
export const getMiscBasic = async () => {
  const url = "/misc/basic";

  return handleFetch<MiscBasicResponse>(url, undefined, {});
};

/**
 * Fetches the current network rate and price-related metrics from the Cexplorer API.
 *
 * This includes data such as ADA to USD rate and other economic indicators.
 *
 * @returns {Promise<MiscRateResponse>} A promise resolving to the rate information object.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getMiscRate = async () => {
  const url = "/misc/rate";
  return handleFetch<MiscRateResponse>(url, undefined, {});
};

/**
 * Fetches constant values used by the Cexplorer API.
 *
 * These constants may include protocol parameters, default limits, or static configuration
 * values necessary for consistent operation of the frontend application.
 *
 * @returns {Promise<MiscConstResponse>} A promise resolving to the constants response.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getMiscConst = async () => {
  const url = "/misc/const";
  return handleFetch<MiscConstResponse>(url, undefined, {});
};

/**
 * Fetches market data for a given epoch number or date.
 *
 * If both `epoch_no` and `date` are provided, the API will prioritize filtering by the available value.
 *
 * @param {number | undefined} epoch_no - Optional epoch number to filter market data by.
 * @param {string | undefined} date - Optional date string (ISO format) to filter market data by.
 *
 * @returns {Promise<MiscMarketResponse>} A promise resolving to the market data response.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getMiscMarket = async (epoch_no: number | undefined, date: string | undefined) => {
  const url = "/misc/market";
  const options = {
    params: {
      epoch_no,
      date,
    },
  };

  return handleFetch<MiscMarketResponse>(url, undefined, options);
};

/**
 * Fetches search results for a given query and optional category and locale.
 *
 * If a category is provided and it belongs to one of the localized categories
 * (`user`, `article`, or `page`), the locale will also be passed in the request.
 *
 * @param {string | undefined} query - The search query string.
 * @param {string} [category] - Optional category to narrow down the search (e.g., "user", "article", "page").
 * @param {Locales} [locale] - Optional locale to be used for localized categories.
 *
 * @returns {Promise<MiscSearchResponse>} A promise that resolves to the search results.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getMiscSearch = (query: string | undefined, category?: string, locale?: Locales) => {
  const url = "/misc/search";
  const localeCategories = ["user", "article", "page"];

  const options = {
    params: {
      query,
      category,
      ...(category &&
        localeCategories.includes(category) && {
          lng: locale,
        }),
    },
  };

  return handleFetch<MiscSearchResponse>(url, undefined, options);
};

/**
 * Fetches the list of governance polls available to the user or publicly.
 *
 * If a `token` is provided, it fetches user-specific governance polls (`/user/gov`),
 * otherwise it fetches the public list (`/misc/gw/gov`).
 *
 * @param {Object} params - Parameters for the request.
 * @param {string} [params.token] - Optional user token to fetch personalized poll data.
 *
 * @returns {Promise<PollListResponse>} A promise that resolves with the list of governance polls.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getPollList = async ({ token }: { token?: string }) => {
  const apiUrl = token ? "/user/gov" : "/misc/gw/gov";

  const options = {
    headers: {
      usertoken: token || "",
    },
  };

  return handleFetch<PollListResponse>(apiUrl, undefined, options);
};

/**
 * Validates a given identifier (`ident`) for a specific group type.
 *
 * This is used to check the correctness or existence of an entity
 * based on its type (e.g., address, policy, etc.).
 *
 * @param {GroupType | undefined} type - The type of the group to validate (e.g., "policy", "address").
 * @param {string} ident - The identifier string to validate.
 *
 * @returns {Promise<MiscValidateResponse>} A promise that resolves with the validation result.
 *
 * @throws Will throw an error if the API request fails.
 */
export const miscValidate = async (type: GroupType | undefined, ident: string) => {
  const url = "/misc/validate";
  const options = {
    params: {
      type,
      ident,
    },
  };

  return handleFetch<MiscValidateResponse>(url, undefined, options);
};

/**
 * Fetches the health status of the Cexplorer API.
 *
 * This endpoint is used for health checks and monitoring purposes.
 *
 * @returns {Promise<MiscHealthResponse>} A promise resolving to the health status.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getMiscHealth = async () => {
  const url = "/misc/health";
  return handleFetch<MiscHealthResponse>(url, undefined, {});
};

/**
 * Fetches the current protocol parameters from the Cardano blockchain.
 *
 * Protocol parameters include various configuration values that govern
 * the behavior of the blockchain.
 *
 * @returns {Promise<MiscProtocolParametersResponse>} A promise resolving to the protocol parameters.
 *
 * @throws Will throw an error if the API request fails.
 */
export const getMiscProtocolParameters = async () => {
  const url = "/misc/protocol_parameters";
  return handleFetch<MiscProtocolParametersResponse>(url, undefined, {});
};
