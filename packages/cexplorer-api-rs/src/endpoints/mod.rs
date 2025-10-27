pub mod block;
pub mod address;

pub use block::{get_block_detail, get_block_list, BlockListParams};
pub use address::{
    get_address_detail, get_address_list, get_address_utxo,
    inspect_address, AddressListParams
};