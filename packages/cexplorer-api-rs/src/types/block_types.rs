use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::pool_types::{PoolInfo, PoolMeta};
use crate::types::epoch_types::EpochParam;
use crate::types::tx_types::{TxBasicInfo, TxInfo, Withdrawal, Mint, AssetRegistry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_no: u64,
    pub time: String,
    pub hash: String,
    pub epoch_no: u64,
    pub slot_no: Option<Option<u64>>,
    pub epoch_slot_no: Option<u64>,
    pub size: Option<u64>,
    pub proto_minor: Option<u64>,
    pub proto_major: Option<u64>,
    pub op_cert_counter: Option<u64>,
    pub vrf_key: Option<String>,
    pub tx_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateCurrency {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub market_cap: f64,
    pub open: f64,
    pub time_close: String,
    pub time_open: String,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rate {
    pub ada: Vec<RateCurrency>,
    pub btc: Vec<RateCurrency>,
    pub date: String,
    pub epoch_no: u64,
    pub fiat: Value,
    pub need_fix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetailResponseDataTxItemMints {
    pub quantity: i64,
    pub policy_id: String,
    pub asset_name: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetailResponseDataTxsItem {
    #[serde(flatten)]
    pub basic_info: TxBasicInfo,
    pub pool: Option<PoolInfo>,
    pub collateral_inputs: Option<Vec<TxInfo>>,
    pub reference_inputs: Option<Vec<TxInfo>>,
    pub all_inputs: Option<Vec<TxInfo>>,
    pub all_collateral_outputs: Option<Value>,
    pub all_outputs: Option<Vec<TxInfo>>,
    pub all_withdrawals: Option<Vec<Withdrawal>>,
    pub mints: Option<Vec<BlockDetailResponseDataTxItemMints>>,
    pub metadata: Option<Value>,
    pub scripts: Option<Value>,
    pub plutus_contracts: Option<Value>,
    pub epoch_param: Option<EpochParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetailResponseData {
    pub txs: Vec<BlockDetailResponseDataTxsItem>,
    pub hash: String,
    pub pool: Option<PoolInfo>,
    pub size: u64,
    pub time: String,
    pub rewards: u64,
    pub slot_no: u64,
    pub vrf_key: Option<String>,
    pub block_no: u64,
    pub epoch_no: u64,
    pub tx_count: u64,
    pub epoch_param: EpochParam,
    pub proto_major: u64,
    pub proto_minor: u64,
    pub epoch_slot_no: u64,
    pub op_cert_counter: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDetailResponse {
    pub code: u64,
    pub data: BlockDetailResponseData,
    pub license: String,
    pub tokens: u64,
    pub ex: f64,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksListResponseDataItem {
    #[serde(flatten)]
    pub block: Block,
    pub pool: Option<PoolInfo>,
    pub epoch_param: EpochParamSimple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochParamSimple {
    pub max_block_size: Option<u64>,
    pub protocol_major: Option<u64>,
    pub protocol_minor: Option<u64>,
    pub max_block_ex_mem: Option<u64>,
    pub max_block_ex_steps: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksListResponseData {
    pub count: u64,
    pub data: Vec<BlocksListResponseDataItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlocksListResponse {
    pub license: String,
    pub code: Option<u64>,
    pub data: BlocksListResponseData,
    pub tokens: i64,
    pub ex: f64,
    pub debug: bool,
}