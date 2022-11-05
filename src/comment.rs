use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use crate::user::User;

pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub title: String,
    pub description: String,
    pub content: String,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
