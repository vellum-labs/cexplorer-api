use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub ticker: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub extended: Option<String>,
    pub homepage: Option<String>,
}
