/**
 * Basic Example: Pool Information
 *
 * This example demonstrates how to fetch stake pool information and performance data.
 */

import { initApi, getPoolsList, getPoolDetail, getPoolDelegators } from '@vellumlabs/cexplorer-api';

async function main() {
  try {
    // Initialize SDK
    initApi({
      network: 'mainnet-stage',
      apiKey: process.env.CEXPLORER_API_KEY || 'your-api-key-here'
    });

    console.log('🏊 Stake Pool Information Example\n');

    // 1. Get list of active pools
    console.log('📋 Fetching active pools...');
    const pools = await getPoolsList({ limit: 10, active: true });

    console.log(`Found ${pools.data.count} active pools`);
    console.log('\nTop 10 pools:');

    for (let i = 0; i < Math.min(5, pools.data.data.length); i++) {
      const pool = pools.data.data[i];
      console.log(`${i + 1}. [${pool.ticker}] ${pool.pool_name || 'Unnamed Pool'}`);
      console.log(`   Pool ID: ${pool.pool_id}`);
      console.log(`   Pledge: ${(pool.pledge / 1000000).toLocaleString()} ADA`);
      console.log(`   Margin: ${pool.margin}%`);
      console.log(`   Blocks: ${pool.block_count}`);
      console.log('');
    }

    // 2. Get detailed information for first pool
    if (pools.data.data.length > 0) {
      const firstPool = pools.data.data[0];
      console.log(`🔍 Detailed information for pool: ${firstPool.ticker}\n`);

      const poolDetail = await getPoolDetail({ pool_id: firstPool.pool_id });

      console.log(`Pool Name: ${poolDetail.data.pool_name || 'N/A'}`);
      console.log(`Description: ${poolDetail.data.description || 'N/A'}`);
      console.log(`Homepage: ${poolDetail.data.homepage || 'N/A'}`);
      console.log(`Saturation: ${(poolDetail.data.saturation * 100).toFixed(2)}%`);
      console.log(`ROA: ${poolDetail.data.roa?.toFixed(4) || 'N/A'}%`);
      console.log(`Fixed Cost: ${(poolDetail.data.fixed_cost / 1000000)} ADA`);

      // 3. Get pool delegators
      console.log(`\n👥 Getting delegators for ${firstPool.ticker}...`);
      const delegators = await getPoolDelegators({
        pool_id: firstPool.pool_id,
        limit: 5
      });

      console.log(`Total delegators: ${delegators.data.count}`);
      console.log('Top 5 delegators:');

      delegators.data.data.forEach((delegator, index) => {
        console.log(`${index + 1}. ${(delegator.amount / 1000000).toLocaleString()} ADA`);
      });
    }

  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

main();