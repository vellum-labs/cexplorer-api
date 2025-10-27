mod error;
mod config;
mod client;
mod endpoints;
pub mod types;

pub use error::CexplorerError;
pub use config::{init_api, get_config, CexplorerConfig};
pub use endpoints::block::{get_block_detail, get_block_list, BlockListParams};
pub use endpoints::address::{
    get_address_detail, get_address_list, get_address_utxo,
    inspect_address, AddressListParams
};
pub use types::{
    BlockDetailResponse, BlocksListResponse,
    AddressDetailResponse, AddressDetailUTXOResponse,
    AddressListResponse, AddressInspectorResponse
};