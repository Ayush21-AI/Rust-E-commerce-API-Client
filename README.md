# E-commerce API Client

A clean, type-safe Rust client library for B2B e-commerce APIs. Built with a focus on safety, performance, and developer experience, this library provides strongly-typed async interfaces for creating and managing orders.

## Features

- **Type Safety**: Strongly typed wrappers for IDs and references prevent common errors
- **Async Support**: Built on `tokio` and `reqwest` for high-performance async I/O
- **Comprehensive Error Handling**: Detailed error types with HTTP status code mapping
- **JSON Serialization**: Robust JSON handling with `serde`
- **Modern TLS**: Uses `rustls` for better performance and security
- **HTTP Basic Authentication**: Built-in support for API authentication

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ecommerce-api-client = "0.1.0"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use ecommerce_api_client::{Client, types::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with authentication
    let client = Client::new("https://api.example.com")?
        .with_credentials("user@example.com", "api_token");

    // Build type-safe order request
    let request = CreateOrderRequest {
        customer_order_reference: Some(CustomerOrderReference("ORDER-001".to_string())),
        order_products: vec![
            CreateOrderProduct {
                product_code: Some(ProductCode("SKU-123".to_string())),
                quantity: 2,
                addressbook: Some(Addressbook {
                    country: "US".to_string(),
                    name: Some("John Doe".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }
        ],
        notes: Some("Urgent order".to_string()),
        ..Default::default()
    };

    // Send request and handle response
    let response = client.create_order(request).await?;
    println!("Order created: {}", response.order.id.0);

    Ok(())
}
```

## Architecture

### Type Safety

The library uses strongly-typed wrappers to prevent common programming errors:

```rust
pub struct OrderId(pub String);
pub struct ProductCode(pub String);
pub struct CustomerOrderReference(pub String);
```

These wrappers are serialized transparently but prevent mixing up different ID types at compile time.

### Error Handling

Comprehensive error types map directly to HTTP status codes:

```rust
#[derive(Error, Debug)]
pub enum Error {
    Http(#[from] reqwest::Error),
    BadRequest(String),      // 400
    Unauthorized(String),    // 401
    NotFound(String),        // 404
    RateLimit(String),       // 429
    ServerError(u16, String), // 5xx
    // ... more variants
}
```

All errors implement `is_retryable()` to help with retry logic.

### JSON Serialization

Smart serialization with optional field handling:

```rust
#[derive(Serialize, Deserialize)]
pub struct CreateOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_order_reference: Option<CustomerOrderReference>,
    pub order_products: Vec<CreateOrderProduct>,
    // ... other fields
}
```

Fields are only included in JSON when they have values, keeping payloads clean.

## Testing

The library includes comprehensive unit tests covering:

- **Serialization**: Verifies requests serialize to correct JSON format
- **Deserialization**: Verifies responses deserialize correctly from JSON
- **Type Safety**: Tests strongly-typed wrappers work as expected
- **Error Handling**: Tests error conditions and status code mapping

Run tests with:

```bash
cargo test
```

## Performance Optimizations

- **rustls-tls**: Modern TLS implementation (faster than OpenSSL bindings)
- **Connection Pooling**: Reuses HTTP connections for better performance
- **Optimized Headers**: Pre-configured headers reduce per-request overhead
- **Efficient JSON**: Uses `serde_json` for fast serialization/deserialization

## Error Recovery

The error types support intelligent retry logic:

```rust
match client.create_order(request).await {
    Ok(response) => println!("Success: {}", response.order.id.0),
    Err(e) => {
        if e.is_retryable() {
            // Implement exponential backoff retry logic
            eprintln!("Retryable error: {}", e);
        } else {
            eprintln!("Permanent error: {}", e);
        }
    }
}
```

## Development

This project follows modern Rust best practices:

- **Zero-Cost Abstractions**: Type wrappers have no runtime overhead
- **Memory Safety**: Safe Rust with proper ownership and borrowing
- **Concurrent Safety**: Thread-safe types where needed
- **Error Ergonomics**: Rich error types with helpful messages
- **Documentation**: Comprehensive docs with examples

## License

MIT License - see LICENSE file for details.