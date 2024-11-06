use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Decompression error: {0}")]
    Decompression(#[from] std::io::Error),
    #[error("Parsing error: {0}")]
    Parsing(#[from] serde_json::Error),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Unexpected status code: {0}")]
    UnexpectedStatusCode(reqwest::StatusCode),
}
