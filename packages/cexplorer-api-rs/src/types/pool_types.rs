
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetaExtended {
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub github_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub reddit_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub twitch_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub discord_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub twitter_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub youtube_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub facebook_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub telegram_handle: Option<String>,
    #[serde(default, deserialize_with = "deserialize_null_string")]
    pub instagram_handle: Option<String>,
}

fn deserialize_null_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMeta {
    pub ticker: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub extended: Option<PoolMetaExtended>,
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub id: String,
    pub meta: Option<PoolMeta>,
}