use once_cell::sync::Lazy;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::error::CexplorerError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CexplorerConfig {
    pub network: String,
    pub api_key: String,
}

static CONFIG: Lazy<Mutex<Option<CexplorerConfig>>> = Lazy::new(|| Mutex::new(None));

pub fn init_api(network: &str, api_key: &str) -> Result<(), CexplorerError> {
    if network.is_empty() {
        return Err(CexplorerError::MissingField("network".to_string()));
    }

    if api_key.is_empty() {
        return Err(CexplorerError::MissingField("api_key".to_string()));
    }

    let config = CexplorerConfig {
        network: network.to_string(),
        api_key: api_key.to_string(),
    };

    let mut cfg = CONFIG.lock().unwrap();
    *cfg = Some(config);

    Ok(())
}

pub fn get_config() -> Result<CexplorerConfig, CexplorerError> {
    let cfg = CONFIG.lock().unwrap();
    cfg.clone().ok_or(CexplorerError::NotInitialized)
}