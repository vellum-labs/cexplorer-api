use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;
use crate::types::user_types::User;
use crate::types::account_types::Meta;
use crate::types::tx_types::AssetRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressTx {
    pub active_epoch_no: u64,
    pub slot: u64,
    pub tx_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDelegation {
    pub pool: String,
    pub tx: AddressTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressStakePool {
    pub id: String,
    pub meta: Meta,
    pub delegation: AddressDelegation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBalance {
    pub live: u64,
    pub active: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressReward {
    pub total: u64,
    pub withdrawn: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressStake {
    pub view: String,
    pub slot_update: u64,
    pub slot_first_registered: u64,
    pub live_pool: AddressStakePool,
    pub active_pool: AddressStakePool,
    pub balance: AddressBalance,
    pub reward: AddressReward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressAssetMarket {
    pub quantity: f64,
    #[serde(rename = "price")]
    pub price_ada: Option<f64>,
    #[serde(rename = "liquidity")]
    pub liquidity_ada: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressAsset {
    pub quantity: u64,
    pub name: String,
    pub registry: Option<AssetRegistry>,
    pub market: AddressAssetMarket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressActivity {
    pub first: String,
    pub recent: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressExtract {
    pub address: String,
    pub magic: u64,
    pub header: u64,
    pub payment: String,
    pub stake: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressAdaHandle {
    pub hex: String,
    pub pkh: String,
    pub utxo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressVoteData {
    pub slot_first_registered: u64,
    pub slot_update: u64,
    pub live_drep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDrepHash {
    pub raw: String,
    pub view: String,
    pub has_script: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDrep {
    pub is_active: bool,
    pub amount: u64,
    pub data: Value,
    pub hash: AddressDrepHash,
    pub since: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressVote {
    pub vote: AddressVoteData,
    pub drep: AddressDrep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDetailData {
    pub address: String,
    pub stake: Option<AddressStake>,
    pub balance: u64,
    pub asset: Vec<AddressAsset>,
    pub activity: AddressActivity,
    pub extract: AddressExtract,
    pub adahandle: Option<AddressAdaHandle>,
    pub user: Option<User>,
    pub vote: Option<AddressVote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDetail {
    pub count: u64,
    pub data: Vec<AddressDetailData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressListItem {
    pub address: String,
    pub asset: Vec<AddressAsset>,
    pub stake: String,
    pub payment_cred: String,
    pub balance: Option<u64>,
    pub first: String,
    pub last: String,
    pub activity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressList {
    pub count: u64,
    pub data: Vec<AddressListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXOAsset {
    pub name: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXOSet {
    pub tx_hash: String,
    pub tx_index: u64,
    pub block_height: u64,
    pub block_time: u64,
    pub value: u64,
    pub datum_hash: Option<String>,
    pub asset_list: Vec<UTXOAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub sum: u64,
    pub has_script: bool,
    pub utxo_set: Vec<UTXOSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressUTXO {
    pub count: u64,
    pub data: Vec<UTXO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInspector {
    pub address: String,
    pub magic: u64,
    pub header: u64,
    pub payment: String,
    pub stake: String,
}

pub type AddressDetailResponse = ResponseCore<AddressDetail>;
pub type AddressDetailUTXOResponse = ResponseCore<AddressUTXO>;
pub type AddressListResponse = ResponseCore<AddressList>;
pub type AddressInspectorResponse = ResponseCore<AddressInspector>;
