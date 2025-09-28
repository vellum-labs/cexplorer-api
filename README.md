# Cexplorer API

[![NPM Version](https://img.shields.io/npm/v/@vellumlabs/cexplorer-api)](https://www.npmjs.com/package/@vellumlabs/cexplorer-api)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue)](https://www.typescriptlang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official API for the [Cexplorer.io](https://beta.cexplorer.io) Cardano blockchain explorer API. This API provides type-safe access to comprehensive Cardano blockchain data including blocks, transactions, addresses, stake pools, DReps, assets, and analytics.

## 🚀 Quick Start

### Installation

```bash
npm install @vellumlabs/cexplorer-api
```

### Get Your API Key

1. Visit [Cexplorer.io](https://beta.cexplorer.io)
2. Sign up with your wallet
3. Navigate to your Profile -> API
4. Generate an API key
5. Copy your API key for use in the SDK

> **⚠️ Important**: API key is required for all requests. The SDK will not work without a valid API key.

### Basic Usage

```typescript
import { initApi, getBlockList, getAddressDetail } from "@vellumlabs/cexplorer-api";

// Initialize the SDK with your API key
initApi({
  network: "mainnet-stage", // or 'preprod-stage', 'preview-stage'
  apiKey: "your-api-key-here",
});

// Get latest blocks
const blocks = await getBlockList({ limit: 10 });
console.log("Latest blocks:", blocks.data.data);

// Get address details
const address = await getAddressDetail({
  address: "addr1qx2kd28nq8ac5prwg32hhvudlwggpgfp8utlyqxu6wqgz62f79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sy0f4qd",
});
console.log("Address balance:", address.data.balance);
```

## 📚 Core Features

### Supported Networks

- **mainnet-stage** - Cardano mainnet
- **preprod-stage** - Preprod testnet
- **preview-stage** - Preview testnet

### Available APIs

| Category                         | Functions                                                                                                                                                                                                                                                                                                                                                       | Description                                   |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| **🏗️ Blocks**                    | `getBlockList`, `getBlockDetail`                                                                                                                                                                                                                                                                                                                                | Block information and transactions            |
| **📍 Addresses**                 | `getAddressDetail`, `getAddressList`, `getAddressUTXO`, `inspectAddress`                                                                                                                                                                                                                                                                                        | Address balances, UTXOs, and history          |
| **💸 Transactions**              | `getTxList`, `getTxDetail`, `getContractTransactions`                                                                                                                                                                                                                                                                                                           | Transaction data and smart contracts          |
| **🏊 Stake Pools**               | `getPoolsList`, `getPoolDetail`, `getPoolDelegators`, `getPoolDelegatorsStats`, `getPoolAwards`, `getPoolBlocks`, `getPoolReward`, `getPoolUpdate`, `getPoolAbout`, `getGlobalPoolAwards`, `getPoolsBirthdays`, `getRetiredPools`, `getTopMarginsWithDelegators`, `getTopMultiDelegators`, `getAveragePool`, `getPoolRegistrations`, `getPoolDeregistrations`   | Pool information, performance, and delegators |
| **🏛️ DReps**                     | `getDrepList`, `getDrepDetail`, `getDrepVote`, `getDrepAnalytics`, `getDrepDelegator`, `getDrepDelegatorStats`, `getDrepStat`, `getAverageDrep`, `getCombinedAverageDrep`, `getDrepNotSpoSameTime`, `getDrepSpoSameTime`, `getStakeDrepRetired`, `getStakeIsSpoDrep`, `getStakeDrepsNotSpo`, `getDrepRegistrations`, `getDrepDeregistrations`, `getDrepUpdates` | Governance representatives data               |
| **🎯 Assets & NFTs**             | `getAssetDetail`, `getAssetList`, `getAssetOwners`, `getNftAssetOwners`, `getAssetMetaData`, `getAssetMint`, `getAssetStats`, `getAssetExchangesGraph`                                                                                                                                                                                                          | Native tokens and NFTs                        |
| **📊 Analytics**                 | `getEpochAnalytics`, `getAnalyticsRate`, `getAnalyticsPoolBlock`, `getAnalyticsStakingAccounts`, `getAnalyticsTopAddresses`, `getAdaPots`, `getWealthComposition`, `getHardforks`, `getGroupDetail`, `getGroupList`                                                                                                                                             | Network statistics and metrics                |
| **🗳️ Governance & Delegation**   | `getDelegationsState`, `getStakeDelegations`, `getDelegationsToRetired`, `checkUserDelegation`, `getDelegEpochChanges`, `getDelegEpochRegistered`                                                                                                                                                                                                               | Delegation and governance data                |
| **📅 Epochs**                    | `getEpochList`, `getEpochDetailStats`, `getEpochDetailParam`                                                                                                                                                                                                                                                                                                    | Epoch information and parameters              |
| **👤 Accounts & Stakes**         | `getAccountRewards`, `getWithrawals`, `getStakeDetail`, `getStakeRegistrations`                                                                                                                                                                                                                                                                                 | Account rewards and stake information         |
| **📜 Scripts & Smart Contracts** | `getScriptList`, `getScriptDetail`, `getScriptDetailRedeemer`                                                                                                                                                                                                                                                                                                   | Plutus scripts and smart contracts            |
| **🏷️ Policies**                  | `getPolicyDetail`, `getPolicyOwner`, `getPolicyStats`                                                                                                                                                                                                                                                                                                           | Minting policies and statistics               |
| **📰 Articles & Content**        | `getArticleList`, `getArticleDetail`                                                                                                                                                                                                                                                                                                                            | Cexplorer articles and content                |
| **💰 Treasury**                  | `getTreasuryDonationStats`                                                                                                                                                                                                                                                                                                                                      | Treasury and donation statistics              |
| **🔄 DeFi & Tokens**             | `getDeFiOrder`, `getDefiTokenList`, `getDefiTokenStat`                                                                                                                                                                                                                                                                                                          | DeFi protocols and token data                 |
| **📋 Metadata**                  | `getMetadataTxList`, `getDatumDetail`                                                                                                                                                                                                                                                                                                                           | Transaction metadata and datums               |
| **🔧 Utilities**                 | `getMiscApi`, `getMiscBasic`, `getMiscConst`, `getMiscMarket`, `getMiscRate`, `getMiscSearch`, `miscValidate`, `getPollList`, `compareWallets`, `getCexplorerConfig`, `initApi`                                                                                                                                                                                 | Utility functions and validation              |

> **📖 Total: 94 functions** across all Cardano blockchain data categories

## 🔧 Configuration

### Networks

```typescript
// Mainnet
initApi({
  network: "mainnet-stage",
  apiKey: "your-api-key",
});

// Testnet
initApi({
  network: "preprod-stage",
  apiKey: "your-api-key",
});
```

### Error Handling

```typescript
try {
  const result = await getBlockList({ limit: 5 });
  console.log(result.data);
} catch (error) {
  if (error.message.includes("API key")) {
    console.error("Invalid or missing API key");
  } else {
    console.error("Request failed:", error.message);
  }
}
```

## 📖 Common Use Cases

### 1. Monitor Latest Blocks

```typescript
const latestBlocks = await getBlockList({
  limit: 20,
  offset: 0,
});

latestBlocks.data.data.forEach((block) => {
  console.log(`Block ${block.block_no}: ${block.tx_count} transactions`);
});
```

### 2. Check Address Balance

```typescript
const addressInfo = await getAddressDetail({
  address: "addr1...",
});

console.log(`Balance: ${addressInfo.data.balance / 1000000} ADA`);
console.log(`Transactions: ${addressInfo.data.tx_count}`);
```

### 3. Get Pool Performance

```typescript
const pool = await getPoolDetail({
  pool_id: "pool1...",
});

console.log(`Pool: ${pool.data.ticker}`);
console.log(`Margin: ${pool.data.margin}%`);
console.log(`Pledge: ${pool.data.pledge / 1000000} ADA`);
```

### 4. Track Asset Information

```typescript
const asset = await getAssetDetail({
  asset: "asset1...",
});

console.log(`Asset: ${asset.data.asset_name}`);
console.log(`Total supply: ${asset.data.total_supply}`);
```

## 🔄 Pagination

Many endpoints support pagination:

```typescript
// Get blocks with pagination
let offset = 0;
const limit = 50;

do {
  const response = await getBlockList({ limit, offset });

  // Process blocks
  response.data.data.forEach((block) => {
    console.log(`Block: ${block.block_no}`);
  });

  offset += limit;

  // Continue if more data available
} while (response.data.data.length === limit);
```

## 🔒 Security Best Practices

### API Key Storage

**❌ Don't:**

```typescript
// Never hardcode API keys in source code
initApi({
  network: "mainnet-stage",
  apiKey: "ak_123456789...", // DON'T DO THIS
});
```

**✅ Do:**

```typescript
// Use environment variables
initApi({
  network: "mainnet-stage",
  apiKey: process.env.CEXPLORER_API_KEY,
});
```

### Environment Variables

Create a `.env` file:

```
CEXPLORER_API_KEY=your-api-key-here
CEXPLORER_NETWORK=mainnet-stage
```

## 🚨 Rate Limits

- Default timeout: 30 seconds
- Automatic retries: 2 attempts
- Respect API rate limits from Cexplorer.io

## 📋 TypeScript Support

Full TypeScript support with comprehensive type definitions:

```typescript
import type { BlocksListResponse, AddressDetailResponse, PoolDetailResponse } from "@vellumlabs/cexplorer-api";

// Type-safe responses
const blocks: BlocksListResponse = await getBlockList({ limit: 10 });
```

## 🐛 Error Handling

The SDK throws descriptive errors:

```typescript
try {
  await getBlockDetail({ hash: "invalid-hash" });
} catch (error) {
  console.error(error.message);
  // Error: The network response failed.
  // Error: Request timed out
  // Error: Missing required "apiKey" in config.
}
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Cexplorer.io](https://beta.cexplorer.io) - Cardano blockchain explorer
- [API Documentation](https://docs.cexplorer.io) - Full API reference
- [NPM Package](https://www.npmjs.com/package/@vellumlabs/cexplorer-api)
- [Issues](https://github.com/vellumlabs/cexplorer-api/issues) - Report bugs

## 📊 More Examples

Check the [examples/](examples/) directory for more comprehensive usage examples:

- [Basic Usage](examples/basic-usage.md)
- [Advanced Queries](examples/advanced-queries.md)
- [Error Handling](examples/error-handling.md)
- [Integration Patterns](examples/integration-patterns.md)

---

Built with ❤️ for the Cardano ecosystem
