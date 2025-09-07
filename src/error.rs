//! Error types for the e-commerce API client

use thiserror::Error;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the API client
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP client errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Invalid URL provided
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    /// Invalid authentication credentials
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),
    
    /// Bad request (400)
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    /// Unauthorized (401)
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    /// Not found (404)
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    
    /// Server error (5xx)
    #[error("Server error {0}: {1}")]
    ServerError(u16, String),
    
    /// Unexpected HTTP status code
    #[error("Unexpected status {0}: {1}")]
    UnexpectedStatus(u16, String),
}

impl Error {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self, 
            Error::Http(_) |
            Error::ServerError(_, _) |
            Error::RateLimit(_)
        )
    }
    
    /// Get HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::BadRequest(_) => Some(400),
            Error::Unauthorized(_) => Some(401),
            Error::NotFound(_) => Some(404),
            Error::RateLimit(_) => Some(429),
            Error::ServerError(code, _) => Some(*code),
            Error::UnexpectedStatus(code, _) => Some(*code),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_retryable() {
        assert!(Error::ServerError(500, "Internal Server Error".to_string()).is_retryable());
        assert!(Error::RateLimit("Too many requests".to_string()).is_retryable());
        assert!(!Error::BadRequest("Invalid request".to_string()).is_retryable());
        assert!(!Error::Unauthorized("Invalid token".to_string()).is_retryable());
    }
    
    #[test]
    fn test_error_status_code() {
        assert_eq!(Error::BadRequest("test".to_string()).status_code(), Some(400));
        assert_eq!(Error::Unauthorized("test".to_string()).status_code(), Some(401));
        assert_eq!(Error::NotFound("test".to_string()).status_code(), Some(404));
        assert_eq!(Error::RateLimit("test".to_string()).status_code(), Some(429));
        assert_eq!(Error::ServerError(503, "test".to_string()).status_code(), Some(503));
        assert_eq!(Error::InvalidUrl("test".to_string()).status_code(), None);
    }
}