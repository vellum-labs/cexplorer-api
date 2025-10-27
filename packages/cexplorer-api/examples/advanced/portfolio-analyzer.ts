/**
 * Advanced Example: Portfolio Analyzer
 *
 * This example demonstrates how to analyze a Cardano wallet portfolio
 * including ADA balance, native tokens, NFTs, and delegation status.
 */

import {
  initApi,
  getAddressDetail,
  getAddressUTXO,
  getAssetDetail,
  checkUserDelegation,
  getPoolDetail,
  compareWallets
} from '@vellumlabs/cexplorer-api';

interface PortfolioAsset {
  asset: string;
  asset_name: string;
  quantity: number;
  policy_id: string;
  decimals?: number;
  isNFT: boolean;
}

interface PortfolioAnalysis {
  address: string;
  adaBalance: number;
  totalAssets: number;
  nftCount: number;
  fungibleTokens: number;
  delegationStatus: any;
  poolInfo?: any;
  assets: PortfolioAsset[];
  riskScore: number;
}

async function analyzePortfolio(address: string): Promise<PortfolioAnalysis> {
  console.log(`🔍 Analyzing portfolio for: ${address.substring(0, 20)}...`);

  // 1. Get basic address information
  const addressInfo = await getAddressDetail({ address });
  const adaBalance = addressInfo.data.balance / 1000000; // Convert to ADA

  console.log(`💰 ADA Balance: ${adaBalance.toLocaleString()} ADA`);

  // 2. Get UTXOs to analyze assets
  const utxos = await getAddressUTXO({ address, limit: 100 });
  const assetMap = new Map<string, PortfolioAsset>();

  console.log(`📦 Analyzing ${utxos.data.data.length} UTXOs...`);

  // Process each UTXO to extract assets
  for (const utxo of utxos.data.data) {
    if (utxo.asset_list) {
      for (const asset of utxo.asset_list) {
        const key = asset.asset;

        if (assetMap.has(key)) {
          assetMap.get(key)!.quantity += asset.quantity;
        } else {
          // Determine if it's an NFT (quantity = 1 and no decimals)
          const isNFT = asset.quantity === 1;

          assetMap.set(key, {
            asset: asset.asset,
            asset_name: asset.asset_name || 'Unknown Asset',
            quantity: asset.quantity,
            policy_id: asset.policy_id,
            isNFT,
            decimals: asset.decimals
          });
        }
      }
    }
  }

  const assets = Array.from(assetMap.values());
  const nftCount = assets.filter(a => a.isNFT).length;
  const fungibleTokens = assets.filter(a => !a.isNFT).length;

  console.log(`🎯 Found ${assets.length} unique assets (${nftCount} NFTs, ${fungibleTokens} tokens)`);

  // 3. Check delegation status
  let delegationStatus = null;
  let poolInfo = null;

  try {
    delegationStatus = await checkUserDelegation({ address });

    if (delegationStatus.data.pool_id) {
      poolInfo = await getPoolDetail({ pool_id: delegationStatus.data.pool_id });
      console.log(`🏊 Delegated to: [${poolInfo.data.ticker}] ${poolInfo.data.pool_name}`);
    } else {
      console.log(`🚫 Not delegating to any pool`);
    }
  } catch (error) {
    console.log(`⚠️ Could not fetch delegation status: ${error.message}`);
  }

  // 4. Calculate risk score (simple heuristic)
  let riskScore = 0;

  // Risk factors:
  if (adaBalance > 10000) riskScore += 20; // Large ADA holding
  if (assets.length > 50) riskScore += 15; // Many different assets
  if (nftCount > 10) riskScore += 10; // Many NFTs
  if (!delegationStatus?.data.pool_id) riskScore += 5; // Not delegating
  if (utxos.data.data.length > 100) riskScore += 10; // Many UTXOs

  console.log(`⚠️ Portfolio Risk Score: ${riskScore}/60`);

  return {
    address,
    adaBalance,
    totalAssets: assets.length,
    nftCount,
    fungibleTokens,
    delegationStatus: delegationStatus?.data,
    poolInfo: poolInfo?.data,
    assets,
    riskScore
  };
}

async function comparePortfolios(addresses: string[]) {
  console.log('\n📊 Comparing Multiple Portfolios\n');

  const portfolios: PortfolioAnalysis[] = [];

  for (const address of addresses) {
    try {
      const portfolio = await analyzePortfolio(address);
      portfolios.push(portfolio);
      console.log('─'.repeat(60));
    } catch (error) {
      console.error(`❌ Failed to analyze ${address}: ${error.message}`);
    }
  }

  // Portfolio comparison summary
  console.log('\n📈 Portfolio Comparison Summary:');
  console.log('════════════════════════════════════════════════════════\n');

  portfolios.sort((a, b) => b.adaBalance - a.adaBalance);

  portfolios.forEach((portfolio, index) => {
    const shortAddr = portfolio.address.substring(0, 15) + '...';
    console.log(`${index + 1}. ${shortAddr}`);
    console.log(`   ADA Balance: ${portfolio.adaBalance.toLocaleString()} ADA`);
    console.log(`   Assets: ${portfolio.totalAssets} (${portfolio.nftCount} NFTs, ${portfolio.fungibleTokens} tokens)`);
    console.log(`   Delegation: ${portfolio.poolInfo?.ticker || 'Not delegating'}`);
    console.log(`   Risk Score: ${portfolio.riskScore}/60`);
    console.log('');
  });

  // Find interesting insights
  const totalADA = portfolios.reduce((sum, p) => sum + p.adaBalance, 0);
  const avgAssets = portfolios.reduce((sum, p) => sum + p.totalAssets, 0) / portfolios.length;
  const highRiskPortfolios = portfolios.filter(p => p.riskScore > 30).length;

  console.log('🔍 Insights:');
  console.log(`• Total ADA across portfolios: ${totalADA.toLocaleString()} ADA`);
  console.log(`• Average assets per portfolio: ${avgAssets.toFixed(1)}`);
  console.log(`• High-risk portfolios (>30 points): ${highRiskPortfolios}`);
  console.log(`• Most diversified: ${portfolios.reduce((max, p) => p.totalAssets > max.totalAssets ? p : max).address.substring(0, 15)}...`);
}

async function main() {
  try {
    // Initialize SDK
    initApi({
      network: 'mainnet-stage',
      apiKey: process.env.CEXPLORER_API_KEY || 'your-api-key-here'
    });

    console.log('💼 Portfolio Analyzer - Advanced Example\n');

    // Sample addresses for analysis (you can replace with real addresses)
    const sampleAddresses = [
      'addr1qx2kd28nq8ac5prwg32hhvudlwggpgfp8utlyqxu6wqgz62f79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sy0f4qd',
      // Add more addresses here for comparison
    ];

    // Analyze first portfolio in detail
    if (sampleAddresses.length > 0) {
      const portfolio = await analyzePortfolio(sampleAddresses[0]);

      // Show top assets
      console.log('\n🏆 Top Assets by Quantity:');
      const topAssets = portfolio.assets
        .filter(a => !a.isNFT)
        .sort((a, b) => b.quantity - a.quantity)
        .slice(0, 10);

      topAssets.forEach((asset, index) => {
        console.log(`${index + 1}. ${asset.asset_name}: ${asset.quantity.toLocaleString()}`);
      });

      // Show NFT collection
      const nfts = portfolio.assets.filter(a => a.isNFT);
      if (nfts.length > 0) {
        console.log(`\n🖼️ NFT Collection (${nfts.length} items):`);
        nfts.slice(0, 5).forEach((nft, index) => {
          console.log(`${index + 1}. ${nft.asset_name}`);
        });
        if (nfts.length > 5) {
          console.log(`   ... and ${nfts.length - 5} more NFTs`);
        }
      }
    }

    // If multiple addresses provided, compare them
    if (sampleAddresses.length > 1) {
      await comparePortfolios(sampleAddresses);
    }

  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

main();