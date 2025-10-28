use serde::{Deserialize, Serialize};
use crate::types::common_types::ResponseCore;
use crate::types::user_types::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleDetailData {
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub article_type: String,
    #[serde(default)]
    pub category: Option<String>,
    pub data: Vec<String>,
    pub pub_date: String,
    #[serde(default)]
    pub mod_date: Option<String>,
    #[serde(default)]
    pub keywords: Option<String>,
    pub description: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    pub render: String,
    #[serde(default)]
    pub mirroring_article: Option<String>,
    pub user_owner: User,
}

pub type ArticleDetailResponse = ResponseCore<ArticleDetailData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleListData {
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub article_type: String,
    pub category: Vec<String>,
    pub pub_date: String,
    pub mod_date: String,
    pub keywords: String,
    pub description: String,
    pub image: String,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    pub user_owner: User,
}

pub type ArticleListResponse = ResponseCore<Vec<ArticleListData>>;
