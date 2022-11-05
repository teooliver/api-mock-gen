use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub title: String,
    pub description: String,
    pub content: String,
    pub user_ref: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
