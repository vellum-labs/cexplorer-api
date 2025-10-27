use crate::config::get_config;
use crate::error::CexplorerError;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

pub async fn fetch<T: DeserializeOwned>(endpoint: &str) -> Result<T, CexplorerError> {
    fetch_with_params::<T, ()>(endpoint, None).await
}

pub async fn fetch_with_params<T: DeserializeOwned, P: Serialize>(
    endpoint: &str,
    params: Option<&P>,
) -> Result<T, CexplorerError> {
    let config = get_config()?;

    let base_url = format!("https://api-{}.cexplorer.io/v1", config.network);
    let url = format!("{}{}", base_url, endpoint);

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let mut request = client
        .get(&url)
        .header("api-key", &config.api_key);

    if let Some(p) = params {
        request = request.query(p);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        return Err(CexplorerError::NetworkError(
            format!("Status: {}", response.status())
        ));
    }


    let text = response.text().await?;

  
    match serde_json::from_str::<T>(&text) {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("JSON parsing error: {}", e);
            eprintln!("Response body (first 5000 chars):\n{}", &text.chars().take(5000).collect::<String>());
            Err(CexplorerError::JsonError(e))
        }
    }
}