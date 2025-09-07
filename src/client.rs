//! HTTP client for the e-commerce API

use crate::error::{Error, Result};
use crate::types::{CreateOrderRequest, CreateOrderResponse};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::time::Duration;

/// HTTP client for interacting with the e-commerce API
#[derive(Debug, Clone)]
pub struct Client {
    /// Base URL for the API
    base_url: String,
    /// HTTP client instance with optimized settings
    http_client: reqwest::Client,
    /// Authentication credentials
    credentials: Option<(String, String)>, // (email, token)
}

impl Client {
    /// Create a new client with the specified base URL
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();
        
        // Validate URL format
        url::Url::parse(&base_url)
            .map_err(|e| Error::InvalidUrl(format!("Invalid base URL: {}", e)))?;
        
        // Build HTTP client with proper configurations
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("ecommerce-api-client/0.1.0"),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .use_rustls_tls()
            .default_headers(headers)
            .build()
            .map_err(Error::Http)?;
        
        Ok(Self {
            base_url,
            http_client,
            credentials: None,
        })
    }
    
    /// Set authentication credentials
    pub fn with_credentials(mut self, email: impl Into<String>, token: impl Into<String>) -> Self {
        self.credentials = Some((email.into(), token.into()));
        self
    }
    
    /// Create a new order
    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<CreateOrderResponse> {
        let url = format!("{}/api_customer/orders", self.base_url);
        
        let mut req_builder = self.http_client
            .post(&url)
            .json(&request);
        
        // Add authentication if configured
        if let Some((email, token)) = &self.credentials {
            let auth_string = format!("{}:{}", email, token);
            let encoded = STANDARD.encode(auth_string.as_bytes());
            req_builder = req_builder.header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Basic {}", encoded))
                    .map_err(|e| Error::InvalidCredentials(format!("Invalid auth header: {}", e)))?
            );
        }
        
        let response = req_builder
            .send()
            .await
            .map_err(Error::Http)?;
        
        // Handle different response status codes
        let status = response.status();
        if status.is_success() {
            response
                .json::<CreateOrderResponse>()
                .await
                .map_err(Error::Http)
        } else {
            let status_code = status.as_u16();
            let error_text = response.text().await.unwrap_or_default();
            
            match status_code {
                400 => Err(Error::BadRequest(error_text)),
                401 => Err(Error::Unauthorized("Invalid credentials".to_string())),
                404 => Err(Error::NotFound("Endpoint not found".to_string())),
                429 => Err(Error::RateLimit("Rate limit exceeded".to_string())),
                500..=599 => Err(Error::ServerError(status_code, error_text)),
                _ => Err(Error::UnexpectedStatus(status_code, error_text)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_creation() {
        let client = Client::new("https://api.example.com").unwrap();
        assert_eq!(client.base_url, "https://api.example.com");
        assert!(client.credentials.is_none());
    }
    
    #[test]
    fn test_client_with_credentials() {
        let client = Client::new("https://api.example.com")
            .unwrap()
            .with_credentials("test@example.com", "token123");
        
        assert!(client.credentials.is_some());
        let (email, token) = client.credentials.unwrap();
        assert_eq!(email, "test@example.com");
        assert_eq!(token, "token123");
    }
    
    #[test]
    fn test_invalid_url() {
        let result = Client::new("not-a-url");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidUrl(_)));
    }
}