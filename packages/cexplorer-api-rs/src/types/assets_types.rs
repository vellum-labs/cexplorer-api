use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRegistry {
    pub ticker: String,
    pub name: String,
    #[serde(default)]
    pub decimals: Option<f64>,
    #[serde(default)]
    pub has_logo: Option<bool>,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyJson {
    #[serde(rename = "type")]
    #[serde(default)]
    pub script_type: Option<String>,
    #[serde(default)]
    pub keyHash: Option<String>,
    #[serde(default)]
    pub slot: Option<f64>,
    #[serde(default)]
    pub required: Option<f64>,
    #[serde(default)]
    pub scripts: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyScript {
    #[serde(rename = "type")]
    pub script_type: String,
    pub json: PolicyJson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetPolicy {
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub mintc: Option<f64>,
    #[serde(default)]
    pub last_mint: Option<String>,
    #[serde(default)]
    pub first_mint: Option<String>,
    #[serde(default)]
    pub script: Option<PolicyScript>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStatInfo {
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub ada_volume: Option<f64>,
    #[serde(default)]
    pub asset_volume: Option<f64>,
    #[serde(default)]
    pub payment_cred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStatAsset {
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub mintc: Option<f64>,
    #[serde(default)]
    pub last_mint: Option<String>,
    #[serde(default)]
    pub first_mint: Option<String>,
    #[serde(default)]
    pub script: Option<PolicyScript>,
    #[serde(default)]
    pub param: Option<Vec<Value>>,
    #[serde(default)]
    pub stats: Option<Vec<AssetStatInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStat {
    #[serde(default)]
    pub asset: Option<AssetStatAsset>,
    #[serde(default)]
    pub policy: Option<AssetPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCore {
    pub name: String,
    #[serde(default)]
    pub registry: Option<AssetRegistry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxAsset {
    pub name: String,
    #[serde(default)]
    pub quantity: Option<f64>,
    #[serde(default)]
    pub registry: Option<AssetRegistry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDexPool {
    pub dex_name: String,
    #[serde(default)]
    pub token_1_amount: Option<f64>,
    #[serde(default)]
    pub token_2_amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDetailDex {
    #[serde(default)]
    pub is_verified: Option<bool>,
    #[serde(default)]
    pub ada_pools: Option<Vec<AssetDexPool>>,
    #[serde(default)]
    pub price_ada: Option<f64>,
    #[serde(default)]
    pub liquidity_ada: Option<f64>,
    #[serde(default)]
    pub stat: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetList {
    pub name: String,
    #[serde(default)]
    pub registry: Option<AssetRegistry>,
    #[serde(default)]
    pub stat: Option<AssetStat>,
    #[serde(default)]
    pub dex: Option<AssetDetailDex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDetail {
    pub name: String,
    #[serde(default)]
    pub fingerprint: Option<String>,
    #[serde(default)]
    pub policy: Option<String>,
    #[serde(default)]
    pub registry: Option<AssetRegistry>,
    #[serde(default)]
    pub stat: Option<AssetStat>,
    #[serde(default)]
    pub dex: Option<AssetDetailDex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetBlock {
    pub hash: String,
    #[serde(default)]
    pub no: Option<f64>,
    pub time: String,
    #[serde(default)]
    pub epoch_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOwner {
    pub address: String,
    #[serde(default)]
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetTx {
    pub hash: String,
    #[serde(default)]
    pub invalid_hereafter: Option<String>,
    pub time: String,
    #[serde(default)]
    pub treasury_donation: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOwnersNftItem {
    pub tx: AssetTx,
    #[serde(default)]
    pub block: Option<AssetBlock>,
    pub owner: AssetOwner,
    #[serde(default)]
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataTx {
    pub hash: String,
    pub time: String,
    #[serde(default)]
    pub invalid_hereafter: Option<f64>,
    #[serde(default)]
    pub treasury_donation: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadataItem {
    #[serde(default)]
    pub key: Option<f64>,
    pub json: Value,
    pub tx: MetadataTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMintAsset {
    pub name: String,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMint {
    pub asset: AssetMintAsset,
    #[serde(default)]
    pub quantity: Option<f64>,
    pub tx: AssetTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStatsStatData {
    #[serde(default)]
    pub count: Option<f64>,
    #[serde(default)]
    pub stake: Option<f64>,
    #[serde(default)]
    pub address: Option<f64>,
    #[serde(default)]
    pub with_data: Option<f64>,
    #[serde(default)]
    pub ada_volume: Option<f64>,
    #[serde(default)]
    pub asset_volume: Option<f64>,
    #[serde(default)]
    pub payment_cred: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetStatsData {
    #[serde(default)]
    pub epoch: Option<f64>,
    #[serde(default)]
    pub stat: Option<Vec<AssetStatsStatData>>,
}

pub type AssetListResponse = ResponseCore<Vec<AssetList>>;
pub type AssetDetailResponse = ResponseCore<AssetDetail>;
pub type AssetOwnersResponse = ResponseCore<Vec<AssetOwner>>;
pub type AssetOwnersNftResponse = ResponseCore<Vec<AssetOwnersNftItem>>;
pub type AssetMetadataResponse = ResponseCore<Vec<AssetMetadataItem>>;
pub type AssetMintResponse = ResponseCore<Vec<AssetMint>>;
pub type AssetStatsResponse = ResponseCore<Vec<AssetStatsData>>;
