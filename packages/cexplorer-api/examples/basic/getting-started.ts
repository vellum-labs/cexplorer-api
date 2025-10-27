/**
 * Basic Example: Getting Started with Cexplorer API
 *
 * This example shows how to initialize the SDK and make your first API calls.
 */

import { initApi, getBlockList, getAddressDetail } from '@vellumlabs/cexplorer-api';

async function main() {
  try {
    // 1. Initialize the SDK with your API key
    initApi({
      network: 'mainnet-stage', // Change to 'preprod-stage' for testnet
      apiKey: process.env.CEXPLORER_API_KEY || 'your-api-key-here'
    });

    console.log('✅ SDK initialized successfully');

    // 2. Get the latest blocks
    console.log('\n📦 Fetching latest blocks...');
    const blocks = await getBlockList({ limit: 5 });

    console.log(`Found ${blocks.data.count} total blocks`);
    blocks.data.data.forEach((block, index) => {
      console.log(`${index + 1}. Block #${block.block_no} - ${block.tx_count} transactions`);
    });

    // 3. Get address details
    console.log('\n💰 Checking address balance...');
    const address = await getAddressDetail({
      address: 'addr1qx2kd28nq8ac5prwg32hhvudlwggpgfp8utlyqxu6wqgz62f79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sy0f4qd'
    });

    console.log(`Address balance: ${(address.data.balance / 1000000).toFixed(6)} ADA`);
    console.log(`Total transactions: ${address.data.tx_count}`);

  } catch (error) {
    console.error('❌ Error:', error.message);

    if (error.message.includes('API key')) {
      console.log('\n💡 How to get API key:');
      console.log('1. Visit https://beta.cexplorer.io');
      console.log('2. Sign up with your wallet');
      console.log('3. Go to Profile -> API');
      console.log('4. Generate an API key');
      console.log('5. Set CEXPLORER_API_KEY environment variable');
    }
  }
}

// Run the example
main();