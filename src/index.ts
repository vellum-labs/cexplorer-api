import { initApi, getCexplorerConfig } from "@/config";

import { getBlockList, getBlockDetail } from "@/endpoints/block";

import { checkUserDelegation, getAccountRewards, getWithrawals } from "@/endpoints/account";

import { getAddressDetail, getAddressList, getAddressUTXO, inspectAddress } from "@/endpoints/address";

import {
  getAdaPots,
  getAnalyticsPoolBlock,
  getAnalyticsRate,
  getAnalyticsStakingAccounts,
  getAnalyticsTopAddresses,
  getAveragePool,
  getEpochAnalytics,
  getGroupDetail,
  getGroupList,
  getHardforks,
  getWealthComposition,
} from "@/endpoints/analytics";

import { getArticleDetail, getArticleList } from "@/endpoints/article";

import {
  getAssetDetail,
  getAssetExchangesGraph,
  getAssetList,
  getAssetMetaData,
  getAssetMint,
  getAssetOwners,
  getAssetStats,
  getNftAssetOwners,
} from "@/endpoints/assets";

import { getDatumDetail } from "@/endpoints/datum";

import { getDelegationsState, getDelegationsToRetired, getStakeDelegations } from "@/endpoints/delegations";

import {
  getAverageDrep,
  getCombinedAverageDrep,
  getDelegEpochChanges,
  getDrepAnalytics,
  getDrepDelegator,
  getDrepDelegatorStats,
  getDrepDetail,
  getDrepList,
  getDrepNotSpoSameTime,
  getDrepSpoSameTime,
  getDrepStat,
  getDrepVote,
  getStakeDrepRetired,
  getStakeIsSpoDrep,
} from "@/endpoints/drep";

import { getEpochDetailParam, getEpochDetailStats, getEpochList } from "@/endpoints/epoch";

import { getMetadataTxList } from "@/endpoints/metadata";

import { getMiscApi, getMiscBasic, getMiscConst, getMiscMarket, getMiscRate, getMiscSearch, getPollList, miscValidate } from "@/endpoints/misc";

import { getPolicyDetail, getPolicyOwner, getPolicyStats } from "@/endpoints/policy";

import {
  getDelegEpochRegistered,
  getGlobalPoolAwards,
  getPoolAbout,
  getPoolAwards,
  getPoolBlocks,
  getPoolDelegators,
  getPoolDelegatorsStats,
  getPoolDetail,
  getPoolReward,
  getPoolUpdate,
  getPoolsBirthdays,
  getPoolsList,
  getRetiredPools,
  getStakeDrepsNotSpo,
  getTopMarginsWithDelegators,
  getTopMultiDelegators,
} from "@/endpoints/pools";

import { getScriptDetail, getScriptDetailRedeemer, getScriptList } from "@/endpoints/scripts";

import { getStakeDetail } from "@/endpoints/stake";

import { getDeFiOrder, getDefiTokenList, getDefiTokenStat } from "@/endpoints/token";

import { getTreasuryDonationStats } from "@/endpoints/treasury";

import {
  getContractTransactions,
  getDrepDeregistrations,
  getDrepRegistrations,
  getDrepUpdates,
  getPoolDeregistrations,
  getPoolRegistrations,
  getStakeRegistrations,
  getTxDetail,
  getTxList,
} from "@/endpoints/tx";

import { compareWallets } from "@/endpoints/wallet";

export {
  initApi,
  inspectAddress,
  checkUserDelegation,
  miscValidate,
  compareWallets,
  getContractTransactions,
  getDrepDeregistrations,
  getDrepRegistrations,
  getDrepUpdates,
  getPoolDeregistrations,
  getPoolRegistrations,
  getStakeRegistrations,
  getTxDetail,
  getTxList,
  getTreasuryDonationStats,
  getDeFiOrder,
  getDefiTokenList,
  getDefiTokenStat,
  getStakeDetail,
  getScriptDetail,
  getScriptDetailRedeemer,
  getScriptList,
  getDelegEpochRegistered,
  getGlobalPoolAwards,
  getPoolAbout,
  getPoolAwards,
  getPoolBlocks,
  getPoolDelegators,
  getPoolDelegatorsStats,
  getPoolDetail,
  getPoolReward,
  getPoolUpdate,
  getPoolsBirthdays,
  getPoolsList,
  getRetiredPools,
  getStakeDrepsNotSpo,
  getTopMarginsWithDelegators,
  getTopMultiDelegators,
  getPolicyDetail,
  getPolicyOwner,
  getPolicyStats,
  getMiscApi,
  getMiscBasic,
  getMiscConst,
  getMiscMarket,
  getMiscRate,
  getMiscSearch,
  getPollList,
  getMetadataTxList,
  getEpochDetailParam,
  getEpochDetailStats,
  getEpochList,
  getAverageDrep,
  getCombinedAverageDrep,
  getDelegEpochChanges,
  getDrepAnalytics,
  getDrepDelegator,
  getDrepDelegatorStats,
  getDrepDetail,
  getDrepList,
  getDrepNotSpoSameTime,
  getDrepSpoSameTime,
  getDrepStat,
  getDrepVote,
  getStakeDrepRetired,
  getStakeIsSpoDrep,
  getDelegationsState,
  getDelegationsToRetired,
  getStakeDelegations,
  getDatumDetail,
  getAssetDetail,
  getAssetExchangesGraph,
  getAssetList,
  getAssetMetaData,
  getAssetMint,
  getAssetOwners,
  getAssetStats,
  getNftAssetOwners,
  getArticleDetail,
  getArticleList,
  getCexplorerConfig,
  getBlockList,
  getBlockDetail,
  getAccountRewards,
  getAdaPots,
  getAddressDetail,
  getAddressList,
  getAddressUTXO,
  getAnalyticsPoolBlock,
  getAnalyticsRate,
  getAnalyticsStakingAccounts,
  getAnalyticsTopAddresses,
  getAveragePool,
  getEpochAnalytics,
  getGroupDetail,
  getGroupList,
  getHardforks,
  getWealthComposition,
  getWithrawals,
};
