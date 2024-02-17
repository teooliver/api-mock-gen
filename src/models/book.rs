use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: String,
    pub cover_image: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
