import type { ArticleDetailResponse, ArticleListResponse } from "@/types/articleTypes";

import { handleFetch } from "@/lib/handleFetch";

interface ArticleDetailProps {
  lng: "en";
  type: "page" | "article";
  url: string;
}

/**
 * Fetches the detail of a specific article from the Cexplorer API.
 *
 * Requires language, type, and article URL as parameters to identify the article.
 *
 * @param {ArticleDetailProps} params - Object containing article parameters.
 * @param {string} params.lng - The language code (e.g. "en", "cs").
 * @param {string} params.type - The article type (e.g. "news", "blog").
 * @param {string} params.url - The unique URL slug of the article.
 *
 * @example
 * ```ts
 * const detail = await getArticleDetail({ lng: "en", type: "news", url: "cardano-updates" });
 * ```
 *
 * @returns {Promise<ArticleDetailResponse>} A promise resolving to the article detail response.
 * @throws If the request fails or the article parameters are invalid.
 */
export const getArticleDetail = async ({ lng, type, url }: ArticleDetailProps) => {
  const apiUrl = "/article/detail";

  const options = {
    params: {
      lng,
      type,
      url,
    },
  };

  return handleFetch<ArticleDetailResponse>(apiUrl, undefined, options);
};

/**
 * Fetches a paginated list of articles from the Cexplorer API.
 *
 * This function retrieves articles in the specified language with optional filtering
 * by category. The result is paginated using `limit` and `offset`.
 *
 * @param {Object} params - Parameters for fetching articles.
 * @param {"en"} params.lng - Language code (currently only "en" is supported).
 * @param {number} params.offset - Offset for pagination.
 * @param {number} params.limit - Number of articles to fetch.
 * @param {string} [params.category] - Optional category filter.
 *
 * @example
 * ```ts
 * const articles = await getArticleList({ lng: "en", offset: 0, limit: 10 });
 * ```
 *
 * @returns {Promise<ArticleListResponse>} A promise resolving to the list of articles.
 * @throws If the request fails or parameters are invalid.
 */
export const getArticleList = async ({ lng, offset, limit, category }: { lng: "en"; offset: number; limit: number; category?: string }) => {
  const apiUrl = "/article/list";

  const options = {
    params: {
      lng,
      type: "article",
      limit,
      offset,
      category,
    },
  };

  return handleFetch<ArticleListResponse>(apiUrl, offset, options);
};
