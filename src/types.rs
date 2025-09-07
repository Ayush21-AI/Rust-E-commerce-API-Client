//! Type-safe data structures for the e-commerce API

use serde::{Deserialize, Serialize};

/// Strongly typed order ID wrapper
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderId(pub String);

/// Strongly typed customer order reference wrapper  
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomerOrderReference(pub String);

/// Strongly typed product code wrapper
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductCode(pub String);

/// Address information for orders
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Addressbook {
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl Default for Addressbook {
    fn default() -> Self {
        Self {
            country: "US".to_string(),
            name: None,
            address: None,
            address2: None,
            city: None,
            province: None,
            postal_code: None,
            phone: None,
            email: None,
            comments: None,
        }
    }
}

/// Product information for order creation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<ProductCode>,
    pub quantity: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressbook: Option<Addressbook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl Default for CreateOrderProduct {
    fn default() -> Self {
        Self {
            product_code: None,
            quantity: 1,
            addressbook: None,
            unit_price: None,
            currency: None,
        }
    }
}

/// Request payload for creating an order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_order_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressbook: Option<Addressbook>,
    pub order_products: Vec<CreateOrderProduct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_customer: Option<String>,
}


/// Order information returned by the API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub status_order_id: u64,
    pub customer_id: u64,
    pub customer_order_reference: String,
    pub gross_total: String,
    pub addressbook_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments_customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_no: Option<String>,
}

/// Order product information from API response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderProduct {
    pub id: u64,
    pub order_id: u64,
    pub product_id: u64,
    pub quantity: String,
    pub price: String,
    pub final_price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressbook_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Response payload from order creation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub order: Order,
    pub order_products: Vec<OrderProduct>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_serialize_create_order_request() {
        let request = CreateOrderRequest {
            customer_order_reference: Some("70000001".to_string()),
            addressbook: Some(Addressbook {
                country: "GB".to_string(),
                ..Default::default()
            }),
            order_products: vec![
                CreateOrderProduct {
                    product_code: Some(ProductCode("274181".to_string())),
                    quantity: 1,
                    addressbook: Some(Addressbook {
                        address: Some("Covent Garden".to_string()),
                        address2: Some("".to_string()),
                        city: Some("London".to_string()),
                        province: Some("".to_string()),
                        postal_code: Some("NR33 7NL".to_string()),
                        country: "GB".to_string(),
                        email: Some("endconsumer@bigecommercewebsite.com".to_string()),
                        name: Some("Test Company".to_string()),
                        phone: Some("0684541247".to_string()),
                        comments: Some("".to_string()),
                    }),
                    ..Default::default()
                },
                CreateOrderProduct {
                    product_code: Some(ProductCode("99999".to_string())),
                    quantity: 1,
                    addressbook: Some(Addressbook {
                        address: Some("Covent Garden".to_string()),
                        address2: Some("".to_string()),
                        city: Some("London".to_string()),
                        province: Some("".to_string()),
                        postal_code: Some("NR33 7NL".to_string()),
                        country: "GB".to_string(),
                        email: Some("endconsumer@bigecommercewebsite.com".to_string()),
                        name: Some("Test Company".to_string()),
                        phone: Some("0684541247".to_string()),
                        comments: Some("".to_string()),
                    }),
                    ..Default::default()
                }
            ],
            comments_customer: None,
        };
        
        let json = serde_json::to_string(&request).unwrap();
        
        // Verify key fields are present in JSON
        assert!(json.contains("70000001"));
        assert!(json.contains("274181"));
        assert!(json.contains("99999"));
        assert!(json.contains("Covent Garden"));
        assert!(json.contains("Test Company"));
        
        // Verify the JSON structure matches the specification
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["customer_order_reference"], "70000001");
        assert_eq!(parsed["addressbook"]["country"], "GB");
        assert_eq!(parsed["order_products"].as_array().unwrap().len(), 2);
        assert_eq!(parsed["order_products"][0]["product_code"], "274181");
        assert_eq!(parsed["order_products"][0]["quantity"], 1);
        assert_eq!(parsed["order_products"][0]["addressbook"]["country"], "GB");
    }
    
    #[test]
    fn test_deserialize_create_order_response() {
        let json_response = r#"{
            "order": {
                "id": 70,
                "status_order_id": 1,
                "customer_id": 9,
                "invoice_no": null,
                "customer_reference_no": 123521478861,
                "comments_customer": "Please deliver asap",
                "customer_order_reference": "74160086",
                "gross_total": "95.97",
                "addressbook_id": 99,
                "created_at": "2018-06-08T03:47:48.000-04:00",
                "updated_at": "2018-06-08T03:47:48.000-04:00"
            },
            "order_products": [
                {
                    "id": 108,
                    "order_id": 70,
                    "product_id": 12646,
                    "quantity": "1.0",
                    "price": "95.97",
                    "final_price": "95.97",
                    "addressbook_id": 100,
                    "created_at": "2018-06-08T03:47:48.000-04:00",
                    "updated_at": "2018-06-08T03:47:48.000-04:00"
                }
            ]
        }"#;
        
        let response: CreateOrderResponse = serde_json::from_str(json_response).unwrap();
        
        // Verify deserialization worked correctly
        assert_eq!(response.order.id, 70);
        assert_eq!(response.order.status_order_id, 1);
        assert_eq!(response.order.customer_id, 9);
        assert_eq!(response.order.customer_order_reference, "74160086");
        assert_eq!(response.order.gross_total, "95.97");
        assert_eq!(response.order.addressbook_id, 99);
        assert_eq!(response.order.created_at.as_ref().unwrap(), "2018-06-08T03:47:48.000-04:00");
        
        // Verify order products array
        assert_eq!(response.order_products.len(), 1);
        let product = &response.order_products[0];
        assert_eq!(product.id, 108);
        assert_eq!(product.order_id, 70);
        assert_eq!(product.product_id, 12646);
        assert_eq!(product.quantity, "1.0");
        assert_eq!(product.price, "95.97");
        assert_eq!(product.final_price, "95.97");
        assert_eq!(product.addressbook_id.unwrap(), 100);
    }
    
    #[test]
    fn test_optional_fields_serialization() {
        let minimal_request = CreateOrderRequest {
            customer_order_reference: None,
            order_products: vec![
                CreateOrderProduct {
                    product_code: Some(ProductCode("SKU-456".to_string())),
                    quantity: 1,
                    addressbook: None,
                    unit_price: None,
                    currency: None,
                }
            ],
            addressbook: None,
            comments_customer: None,
        };
        
        let json = serde_json::to_string(&minimal_request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        
        // Verify optional fields are not present when None
        assert!(parsed.get("customer_order_reference").is_none());
        assert!(parsed.get("addressbook").is_none());
        assert!(parsed.get("comments_customer").is_none());
        assert!(parsed["order_products"][0].get("unit_price").is_none());
        assert!(parsed["order_products"][0].get("addressbook").is_none());
    }
    
    #[test] 
    fn test_strongly_typed_wrappers() {
        let order_id = OrderId("ord_123".to_string());
        let customer_ref = CustomerOrderReference("ORDER-001".to_string());
        let product_code = ProductCode("SKU-456".to_string());
        
        // Test serialization of wrappers
        assert_eq!(serde_json::to_string(&order_id).unwrap(), "\"ord_123\"");
        assert_eq!(serde_json::to_string(&customer_ref).unwrap(), "\"ORDER-001\"");
        assert_eq!(serde_json::to_string(&product_code).unwrap(), "\"SKU-456\"");
        
        // Test deserialization of wrappers
        assert_eq!(serde_json::from_str::<OrderId>("\"ord_456\"").unwrap().0, "ord_456");
        assert_eq!(serde_json::from_str::<CustomerOrderReference>("\"ORDER-002\"").unwrap().0, "ORDER-002");
        assert_eq!(serde_json::from_str::<ProductCode>("\"SKU-789\"").unwrap().0, "SKU-789");
    }
    
    #[test]
    fn test_address_default() {
        let address = Addressbook::default();
        assert_eq!(address.country, "US");
        assert!(address.name.is_none());
        assert!(address.address.is_none());
    }
}