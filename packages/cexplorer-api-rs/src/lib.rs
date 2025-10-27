mod error;
mod config;
mod client;
mod endpoints;
pub mod types;

pub use error::CexplorerError;
pub use config::{init_api, get_config, CexplorerConfig};
pub use client::fetch;
pub use endpoints::block::{get_block_detail, get_block_list, BlockListParams};
pub use types::{BlockDetailResponse, BlocksListResponse};