/**
 * Basic Example: Asset and NFT Tracking
 *
 * This example shows how to work with native tokens and NFTs on Cardano.
 */

import {
  initApi,
  getAssetList,
  getAssetDetail,
  getAssetOwners,
  getNftAssetOwners,
  getPolicyDetail
} from '@vellumlabs/cexplorer-api';

async function main() {
  try {
    // Initialize SDK
    initApi({
      network: 'mainnet-stage',
      apiKey: process.env.CEXPLORER_API_KEY || 'your-api-key-here'
    });

    console.log('🎯 Asset and NFT Tracking Example\n');

    // 1. Get list of assets
    console.log('📋 Fetching recent assets...');
    const assets = await getAssetList({ limit: 10 });

    console.log(`Found ${assets.data.count} total assets`);
    console.log('\nRecent assets:');

    for (let i = 0; i < Math.min(5, assets.data.data.length); i++) {
      const asset = assets.data.data[i];
      const assetName = asset.asset_name || 'Unnamed Asset';

      console.log(`${i + 1}. ${assetName}`);
      console.log(`   Asset: ${asset.asset}`);
      console.log(`   Policy: ${asset.policy_id}`);
      console.log(`   Supply: ${asset.total_supply?.toLocaleString() || 'N/A'}`);
      console.log('');
    }

    // 2. Get detailed asset information
    if (assets.data.data.length > 0) {
      const firstAsset = assets.data.data[0];
      console.log(`🔍 Detailed information for: ${firstAsset.asset_name || 'Asset'}\n`);

      const assetDetail = await getAssetDetail({ asset: firstAsset.asset });

      console.log(`Name: ${assetDetail.data.asset_name || 'N/A'}`);
      console.log(`Fingerprint: ${assetDetail.data.fingerprint || 'N/A'}`);
      console.log(`Total Supply: ${assetDetail.data.total_supply?.toLocaleString() || 'N/A'}`);
      console.log(`Decimals: ${assetDetail.data.decimals || 0}`);
      console.log(`First Mint Block: ${assetDetail.data.first_mint_block || 'N/A'}`);
      console.log(`Mint Count: ${assetDetail.data.mint_count || 0}`);

      // 3. Get asset owners
      console.log(`\n👥 Getting owners for this asset...`);
      const owners = await getAssetOwners({
        asset: firstAsset.asset,
        limit: 5
      });

      console.log(`Total owners: ${owners.data.count}`);
      console.log('Top 5 holders:');

      owners.data.data.forEach((owner, index) => {
        console.log(`${index + 1}. ${owner.address.substring(0, 20)}... - ${owner.quantity.toLocaleString()}`);
      });

      // 4. Get policy information
      console.log(`\n📜 Policy information...`);
      const policy = await getPolicyDetail({ policy_id: firstAsset.policy_id });

      console.log(`Policy ID: ${policy.data.policy_id}`);
      console.log(`Asset Count: ${policy.data.asset_count || 0}`);
      console.log(`Holders Count: ${policy.data.holders_count || 0}`);
    }

    // 5. Example with a known NFT policy (Clay Nation)
    console.log('\n🖼️ NFT Example - Clay Nation Collection');
    try {
      const clayNationPolicy = '40fa2aa67258b4ce7b5782f74831d46a84c59a0ff0c28262fab21728';

      const nftOwners = await getNftAssetOwners({
        policy_id: clayNationPolicy,
        limit: 3
      });

      console.log(`NFT holders in collection: ${nftOwners.data.count}`);
      console.log('Sample NFT holders:');

      nftOwners.data.data.forEach((holder, index) => {
        console.log(`${index + 1}. ${holder.address.substring(0, 20)}... owns ${holder.quantity} NFTs`);
      });

    } catch (error) {
      console.log('Note: NFT example failed (policy might not exist or be accessible)');
    }

  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

main();