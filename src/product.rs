use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub brand: String,
    pub images: Vec<String>,
    pub description: String,
    pub price: Vec<ProductPrice>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductPrice {
    pub country: String,
    pub price: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductReview {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_ref: Uuid,
    pub product_ref: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
