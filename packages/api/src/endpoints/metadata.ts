import type { MetadataTxListResponse } from "@/types/metadataTypes";

import { handleFetch } from "@/lib/handleFetch";

interface MetadataTxListParams {
  limit?: number;
  offset?: number;
  tx?: string;
  key?: number;
}

/**
 * Fetches a list of transactions containing metadata based on provided filters.
 *
 * @param {MetadataTxListParams} params - The parameters for the request.
 * @param {number} [params.limit=10] - Maximum number of records to return.
 * @param {number} [params.offset=0] - Offset for pagination.
 * @param {string} [params.tx] - Transaction hash to filter by.
 * @param {string} [params.key] - Metadata key to filter by.
 *
 * @returns {Promise<MetadataTxListResponse>} A promise resolving to the list of metadata transactions.
 *
 * @throws Will throw an error if the fetch fails or parameters are invalid.
 */
export const getMetadataTxList = async ({ limit = 10, offset = 0, tx, key }: MetadataTxListParams) => {
  const url = `/metadata/list`;
  const options = {
    params: { limit, offset, tx, key },
  };

  return handleFetch<MetadataTxListResponse>(url, offset, options);
};
