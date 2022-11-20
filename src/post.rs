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
use crate::user::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PostStatus {
    Draft,
    Published,
}

impl PostStatus {
    fn get_random_post_status() -> PostStatus {
        // TODO: use the following when stable so we dont have to
        // hard code the enum length in the `gen_range`, that way we can
        // avoid breaking functionality with the enum changes.
        //
        // https://github.com/rust-lang/rust/issues/73662
        // let enum_length = mem::variant_count::<TaskStatus>();

        match rand::thread_rng().gen_range(0..=1) {
            0 => PostStatus::Draft,
            1 => PostStatus::Published,
            _ => PostStatus::Draft,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub content: String,
    pub status: PostStatus,
    pub user_ref: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
}

impl Post {
    pub fn new_random_post(user: &Option<User>) -> Post {
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

        Post {
            id: Uuid::new_v4(),
            title: Words(3..5).fake::<Vec<String>>().join(" "),
            description: Words(5..10).fake::<Vec<String>>().join(" "),
            content: Words(10..30).fake::<Vec<String>>().join(" "),
            status: PostStatus::get_random_post_status(),
            user_ref: user.id,
            created_at: fake_initial_date,
            updated_at: fake_end_date,
            published_at: fake_end_date,
        }
    }
}
