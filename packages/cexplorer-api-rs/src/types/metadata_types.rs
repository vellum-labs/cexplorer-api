use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::common_types::ResponseCore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataTx {
    pub hash: String,
    #[serde(default)]
    pub slot_no: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataTxListItem {
    pub tx: MetadataTx,
    #[serde(default)]
    pub key: Option<f64>,
    pub md: Value,
    #[serde(default)]
    pub size: Option<f64>,
}

pub type MetadataTxListResponse = ResponseCore<Vec<MetadataTxListItem>>;
