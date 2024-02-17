use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub director: String,
    pub poster: String,
    pub actors: Vec<Actor>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Actor {
    pub id: Uuid,
    pub name: String,
    pub last_name: String,
    pub picture: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
