//! # E-commerce API Client
//!
//! A clean, type-safe Rust client library for B2B e-commerce APIs.
//! Provides strongly-typed, async interfaces for creating and managing orders.
//!
//! ## Features
//!
//! - **Type Safety**: Strongly typed wrappers for IDs and references
//! - **Async Support**: Built on `tokio` and `reqwest` for high-performance async I/O
//! - **Error Handling**: Comprehensive error types with detailed context
//! - **Serialization**: Robust JSON handling with `serde`
//! - **Authentication**: HTTP Basic authentication support
//!
//! ## Quick Start
//!
//! ```rust
//! use ecommerce_api_client::{Client, types::*};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("https://api.example.com")?
//!     .with_credentials("user@example.com", "api_token");
//!
//! let request = CreateOrderRequest {
//!     customer_order_reference: Some("ORDER-001".to_string()),
//!     order_products: vec![
//!         CreateOrderProduct {
//!             product_code: Some(ProductCode("SKU-123".to_string())),
//!             quantity: 1,
//!             addressbook: Some(Addressbook {
//!                 country: "US".to_string(),
//!                 name: Some("John Doe".to_string()),
//!                 ..Default::default()
//!             }),
//!             ..Default::default()
//!         }
//!     ],
//!     ..Default::default()
//! };
//!
//! let response = client.create_order(request).await?;
//! println!("Order created with ID: {}", response.order.id);
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod types;

pub use client::Client;
pub use error::{Error, Result};

/// Re-export commonly used types for convenience
pub mod prelude {
    pub use crate::client::Client;
    pub use crate::error::{Error, Result};
    pub use crate::types::{
        CreateOrderRequest, CreateOrderResponse, CreateOrderProduct,
        Addressbook, CustomerOrderReference, ProductCode, OrderId,
    };
}