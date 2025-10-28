use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareWalletPlatforms {
    #[serde(rename = "iOS")]
    pub ios: Value,
    pub web: Value,
    pub android: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareWalletCompatibility {
    pub ledger: Value,
    pub trezor: Value,
    pub keystone: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletPartner {
    pub enabled: Value,
    pub partner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainCompatibility {
    pub enabled: Value,
    #[serde(default)]
    pub supportedChains: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareWallet {
    #[serde(rename = "customNode")]
    pub custom_node: Value,
    #[serde(rename = "fiatOnramp")]
    pub fiat_onramp: WalletPartner,
    pub opensource: Value,
    #[serde(rename = "addressBook")]
    pub address_book: Value,
    #[serde(rename = "dAppBrowser")]
    pub d_app_browser: Value,
    #[serde(rename = "internalName")]
    pub internal_name: String,
    #[serde(rename = "otherFeatures")]
    #[serde(default)]
    pub other_features: Option<Value>,
    #[serde(rename = "swapsInWallet")]
    pub swaps_in_wallet: WalletPartner,
    #[serde(rename = "governanceInfo")]
    pub governance_info: String,
    #[serde(rename = "stakingSupport")]
    pub staking_support: Value,
    #[serde(rename = "testnetSupport")]
    pub testnet_support: Value,
    #[serde(rename = "multipleAccounts")]
    pub multiple_accounts: Value,
    #[serde(rename = "governanceSupport")]
    pub governance_support: Value,
    #[serde(rename = "supportedPlatforms")]
    pub supported_platforms: CompareWalletPlatforms,
    #[serde(rename = "multiPoolDelegation")]
    pub multi_pool_delegation: Value,
    #[serde(rename = "crossChainCompatibility")]
    pub cross_chain_compatibility: CrossChainCompatibility,
    #[serde(rename = "smartContractInteraction")]
    pub smart_contract_interaction: Value,
    #[serde(rename = "nftMarketplaceIntegration")]
    pub nft_marketplace_integration: WalletPartner,
    #[serde(rename = "hardwareWalletCompatibility")]
    pub hardware_wallet_compatibility: HardwareWalletCompatibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareWallets {
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub wallet_type: String,
    #[serde(default)]
    pub category: Option<String>,
    pub pub_date: String,
    #[serde(default)]
    pub mod_date: Option<String>,
    pub keywords: String,
    pub description: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    pub render: String,
    #[serde(default)]
    pub mirroring_article: Option<String>,
    pub data: Vec<Vec<CompareWallet>>,
}

pub type CompareWalletsResponse = ResponseCore<CompareWallets>;
