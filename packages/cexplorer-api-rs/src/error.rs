use thiserror::Error;


#[derive(Error, Debug)]
pub enum CexplorerError {
    #[error("SDK not initialized. Call initApi() first")]
    NotInitialized,

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid API key format")]
    InvalidApiKey,

    #[error("Network request failed: {0}")]
    NetworkError(String),

    #[error("Request timeout")]
    Timeout,

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
}