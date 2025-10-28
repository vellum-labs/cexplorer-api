use crate::client::fetch_with_params;
use crate::error::CexplorerError;
use crate::types::article_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleDetailParams {
    pub lng: String,
    #[serde(rename = "type")]
    pub article_type: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleListParams {
    pub lng: String,
    #[serde(rename = "type")]
    pub article_type: String,
    pub limit: u64,
    pub offset: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

pub async fn get_article_detail(
    lng: &str,
    article_type: &str,
    url: &str,
) -> Result<ArticleDetailResponse, CexplorerError> {
    let endpoint = "/article/detail";
    let params = ArticleDetailParams {
        lng: lng.to_string(),
        article_type: article_type.to_string(),
        url: url.to_string(),
    };
    fetch_with_params::<ArticleDetailResponse, ArticleDetailParams>(endpoint, Some(&params)).await
}

pub async fn get_article_list(
    lng: &str,
    offset: u64,
    limit: u64,
    category: Option<&str>,
) -> Result<ArticleListResponse, CexplorerError> {
    let endpoint = "/article/list";
    let params = ArticleListParams {
        lng: lng.to_string(),
        article_type: "article".to_string(),
        limit,
        offset,
        category: category.map(|s| s.to_string()),
    };
    fetch_with_params::<ArticleListResponse, ArticleListParams>(endpoint, Some(&params)).await
}
