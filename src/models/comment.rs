use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use fake::faker::lorem::en::Words;
use fake::Fake;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::helpers::TIME_IN_SECONDS_OPTIONS;

use super::Post;
use super::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl Comment {
    pub fn new_random_comment(user: &Option<User>, post: &Post) -> Comment {
        let user = match user {
            Some(u) => u.clone(),
            None => User::new_random_user(),
        };

        let random_amount_of_days = rand::thread_rng().gen_range(0..=10);
        let amount_of_days = Duration::days(random_amount_of_days);
        let random_time_in_seconds =
            TIME_IN_SECONDS_OPTIONS[rand::thread_rng().gen_range(0..TIME_IN_SECONDS_OPTIONS.len())];

        let random_date = Utc::now() - amount_of_days;

        let fake_initial_date = random_date - Duration::seconds(random_time_in_seconds as i64);
        let fake_end_date = random_date + Duration::seconds(random_time_in_seconds as i64);

        Comment {
            id: Uuid::new_v4(),
            title: Words(3..5).fake::<Vec<String>>().join(" "),
            post_id: post.id,
            description: Words(5..10).fake::<Vec<String>>().join(" "),
            content: Words(10..30).fake::<Vec<String>>().join(" "),
            user_ref: user.id,
            created_at: fake_initial_date,
            updated_at: fake_end_date,
        }
    }
}
