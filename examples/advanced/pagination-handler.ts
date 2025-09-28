/**
 * Advanced Example: Pagination Handler
 *
 * This example demonstrates how to handle pagination efficiently
 * when fetching large datasets from the Cexplorer API.
 */

import { initApi, getBlockList, getAddressList, getTxList } from '@vellumlabs/cexplorer-api';

interface PaginationOptions {
  limit?: number;
  maxPages?: number;
  onProgress?: (page: number, total: number) => void;
}

/**
 * Generic pagination handler for any Cexplorer endpoint
 */
async function fetchAllPages<T extends { data: { data: any[], count: number } }>(
  fetchFunction: (params: any) => Promise<T>,
  baseParams: any = {},
  options: PaginationOptions = {}
): Promise<T['data']['data']> {
  const { limit = 100, maxPages = Infinity, onProgress } = options;
  const allData: T['data']['data'] = [];
  let offset = 0;
  let page = 1;
  let totalCount = 0;

  while (page <= maxPages) {
    try {
      const response = await fetchFunction({
        ...baseParams,
        limit,
        offset
      });

      const { data, count } = response.data;
      allData.push(...data);
      totalCount = count;

      // Calculate total pages for progress reporting
      const totalPages = Math.ceil(totalCount / limit);

      if (onProgress) {
        onProgress(page, totalPages);
      }

      console.log(`📄 Page ${page}/${totalPages} - Fetched ${data.length} items (Total: ${allData.length}/${totalCount})`);

      // Break if we've fetched all available data
      if (data.length < limit || allData.length >= totalCount) {
        break;
      }

      offset += limit;
      page++;

      // Add a small delay to be respectful to the API
      await new Promise(resolve => setTimeout(resolve, 100));

    } catch (error) {
      console.error(`❌ Error on page ${page}:`, error.message);
      break;
    }
  }

  return allData;
}

async function main() {
  try {
    // Initialize SDK
    initApi({
      network: 'mainnet-stage',
      apiKey: process.env.CEXPLORER_API_KEY || 'your-api-key-here'
    });

    console.log('📖 Advanced Pagination Example\n');

    // Example 1: Fetch all blocks from the last epoch
    console.log('1️⃣ Fetching blocks with pagination...');

    const blocks = await fetchAllPages(
      getBlockList,
      {}, // No additional filters
      {
        limit: 50,
        maxPages: 5, // Limit to 5 pages for demo
        onProgress: (page, total) => {
          const progress = ((page / total) * 100).toFixed(1);
          console.log(`   Progress: ${progress}%`);
        }
      }
    );

    console.log(`✅ Fetched ${blocks.length} blocks total\n`);

    // Example 2: Find all addresses with specific characteristics
    console.log('2️⃣ Finding addresses with pagination...');

    const addresses = await fetchAllPages(
      getAddressList,
      { min_balance: 1000000000 }, // Min 1000 ADA
      {
        limit: 20,
        maxPages: 3
      }
    );

    console.log(`✅ Found ${addresses.length} addresses with >1000 ADA\n`);

    // Example 3: Batch processing with pagination
    console.log('3️⃣ Batch processing transactions...');

    let processedCount = 0;
    const batchSize = 25;

    const transactions = await fetchAllPages(
      getTxList,
      {},
      {
        limit: batchSize,
        maxPages: 4,
        onProgress: (page, total) => {
          console.log(`   Processing batch ${page} of ${total}`);
        }
      }
    );

    // Process transactions in batches
    for (let i = 0; i < transactions.length; i += batchSize) {
      const batch = transactions.slice(i, i + batchSize);

      // Simulate processing
      batch.forEach(tx => {
        // Process each transaction
        processedCount++;
      });

      console.log(`   Processed ${processedCount}/${transactions.length} transactions`);
    }

    console.log(`✅ Processed ${processedCount} transactions total\n`);

    // Example 4: Smart pagination with error handling
    console.log('4️⃣ Smart pagination with retry logic...');

    const fetchWithRetry = async (fetchFn: any, params: any, retries = 3): Promise<any> => {
      for (let attempt = 1; attempt <= retries; attempt++) {
        try {
          return await fetchFn(params);
        } catch (error) {
          console.log(`   Attempt ${attempt} failed: ${error.message}`);

          if (attempt === retries) {
            throw error;
          }

          // Exponential backoff
          const delay = Math.pow(2, attempt) * 1000;
          console.log(`   Retrying in ${delay}ms...`);
          await new Promise(resolve => setTimeout(resolve, delay));
        }
      }
    };

    const robustBlocks = await fetchAllPages(
      (params) => fetchWithRetry(getBlockList, params),
      {},
      { limit: 30, maxPages: 2 }
    );

    console.log(`✅ Fetched ${robustBlocks.length} blocks with retry logic\n`);

    // Summary
    console.log('📊 Pagination Summary:');
    console.log(`• Total blocks fetched: ${blocks.length}`);
    console.log(`• Addresses found: ${addresses.length}`);
    console.log(`• Transactions processed: ${processedCount}`);
    console.log(`• Robust blocks: ${robustBlocks.length}`);

  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

main();