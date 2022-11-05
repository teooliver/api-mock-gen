use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use crate::user::User;

pub enum PostStatus {
    Draft,
    Published,
}

pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub content: String,
    pub status: PostStatus,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
}
