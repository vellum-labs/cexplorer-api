use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCore<T> {
    pub code: u64,
    pub data: T,
    pub tokens: u64,
    pub ex: f64,
    pub debug: bool,
}
