import type { AddressDetailResponse, AddressDetailUTXOResponse, AddressInspectorResponse, AddressListResponse } from "@/types/addressTypes";

import { handleFetch } from "@/lib/handleFetch";

interface AddressDetailParams {
  view?: string;
}

/**
 * Fetches detailed information for a specific address from the Cexplorer API.
 *
 * Sends a request to retrieve data about the given address, such as balance,
 * transaction count, and other metadata.
 *
 * @example
 * ```ts
 * const detail = await getAddressDetail({ view: "addr1..." });
 * ```
 *
 * @param {AddressDetailParams} args - Parameters for the request.
 * @param {string} args.view - The address to fetch details for.
 *
 * @returns {Promise<AddressDetailResponse>} A promise resolving to the address detail response.
 * @throws If the request fails or returns an invalid response.
 */

export const getAddressDetail = async ({ view }: AddressDetailParams) => {
  const url = `/address/detail?view=${view}`;

  return handleFetch<AddressDetailResponse>(url);
};

/**
 * Fetches a list of addresses from the Cexplorer API based on provided filters.
 *
 * This endpoint supports filtering by payment credential, full address, sorting options,
 * and watchlist inclusion. A user token can be provided via headers to access user-specific data.
 *
 * @example
 * ```ts
 * const addresses = await getAddressList({
 *   payment: "some_payment_cred",
 *   view: "addr1...",
 *   order: "balance",
 *   watchlist_only: "1",
 *   token: "user-token"
 * });
 * ```
 *
 * @param {Object} params - Filter and configuration options.
 * @param {string} [params.payment] - Stake key or payment credential to filter addresses.
 * @param {string} [params.view] - Specific address to filter.
 * @param {"balance" | "last"} [params.order] - Sorting method: by balance or last activity.
 * @param {"1"} [params.watchlist_only] - If set to "1", only returns watchlist addresses.
 * @param {string} [params.token] - Optional user token for authenticated requests.
 *
 * @returns {Promise<AddressListResponse>} A promise resolving to the filtered list of addresses.
 * @throws If the fetch request fails or returns an error response.
 */

export const getAddressList = async ({
  payment,
  view,
  order,
  watchlist_only,
  token,
}: {
  payment?: string;
  view?: string;
  order?: "balance" | "last";
  watchlist_only?: "1" | undefined;
  token?: string;
}) => {
  const url = `/address/list`;
  const options = {
    params: { payment_cred: payment, view, order, watchlist_only },
    headers: {
      usertoken: token ?? "",
    },
  };

  return handleFetch<AddressListResponse>(url, undefined, options);
};

/**
 * Fetches the list of UTXOs (Unspent Transaction Outputs) for a given address.
 *
 * This function queries the `/address/utxo` endpoint of the Cexplorer API to retrieve
 * all unspent outputs associated with the provided address (view).
 *
 * @example
 * ```ts
 * const utxos = await getAddressUTXO({ view: "addr1..." });
 * ```
 *
 * @param {AddressDetailParams} params - Parameters for the request.
 * @param {string} params.view - The address for which to fetch UTXO data.
 *
 * @returns {Promise<AddressDetailUTXOResponse>} A promise resolving to the address's UTXO information.
 * @throws If the fetch request fails or returns an error response.
 */

export const getAddressUTXO = async ({ view }: AddressDetailParams) => {
  const url = `/address/utxo?view=${view}`;

  return handleFetch<AddressDetailUTXOResponse>(url);
};

/**
 * Retrieves detailed information extracted from a given Cardano address.
 *
 * This function queries the `/address/extract` endpoint of the Cexplorer API to parse
 * and return metadata about the provided address, such as its type, network, and other properties.
 *
 * @example
 * ```ts
 * const result = await inspectAddress({ view: "addr1..." });
 * ```
 *
 * @param {AddressDetailParams} params - Parameters for the request.
 * @param {string} params.view - The address to inspect and extract data from.
 *
 * @returns {Promise<AddressInspectorResponse>} A promise resolving to the parsed address information.
 * @throws If the fetch request fails or the API returns an error.
 */

export const inspectAddress = async ({ view }: AddressDetailParams) => {
  const url = `/address/extract?view=${view}`;

  return handleFetch<AddressInspectorResponse>(url);
};
