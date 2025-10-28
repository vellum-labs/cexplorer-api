use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatumCore<T> {
    pub code: f64,
    pub data: T,
    #[serde(default)]
    pub tokens: Option<f64>,
    #[serde(default)]
    pub ex: Option<f64>,
    #[serde(default)]
    pub debug: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatumHash {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatumFields {
    #[serde(default)]
    pub fields: Option<Vec<Value>>,
    #[serde(default)]
    pub constructor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatumValue {
    #[serde(default)]
    pub fields: Option<Vec<DatumFields>>,
    #[serde(default)]
    pub constructor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatumDetailData {
    pub hash: String,
    pub datum: String,
    #[serde(default)]
    pub tx: Option<Vec<DatumHash>>,
    #[serde(default)]
    pub datums_in_same_tx: Option<Vec<DatumHash>>,
    #[serde(default)]
    pub value: Option<DatumValue>,
}

pub type DatumDetailResponse = DatumCore<DatumDetailData>;
