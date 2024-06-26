use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::Rng;
use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

use crate::helpers::PROJECT_COLORS;
use crate::helpers::TIME_IN_SECONDS_OPTIONS;

use super::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TaskStatus {
    Done,
    InProgress,
    NotNedeed,
    ReadyToStart,
    Backlog,
}

impl From<&str> for TaskStatus {
    fn from(s: &str) -> TaskStatus {
        match s {
            "Done" => TaskStatus::Done,
            "InProgress" => TaskStatus::InProgress,
            "NotNeeded" => TaskStatus::NotNedeed,
            "ReadyToStart" => TaskStatus::ReadyToStart,
            _ => TaskStatus::Backlog,
        }
    }
}

impl TaskStatus {
    fn get_random_task_status() -> TaskStatus {
        // TODO: upddate to `variant_count` when stable so we dont have to
        // hard code the enum length in the `gen_range`, that way we can
        // avoid breaking functionality with the enum changes.
        //
        // https://github.com/rust-lang/rust/issues/73662
        // let enum_length = mem::variant_count::<TaskStatus>();

        match rand::thread_rng().gen_range(0..=3) {
            0 => TaskStatus::Done,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::NotNedeed,
            3 => TaskStatus::ReadyToStart,
            _ => TaskStatus::Backlog,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub user_ref: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub color: Option<String>,
}

impl Task {
    pub fn new_random_task(user: &Option<User>) -> Task {
        let user_ref = match user {
            Some(u) => u.clone().id,
            None => User::new_random_user().id,
        };
        let color = rand::thread_rng()
            .gen_range(0..(PROJECT_COLORS.len() - 1))
            .to_string();

        let random_amount_of_days = rand::thread_rng().gen_range(0..=10);
        let amount_of_days = Duration::days(random_amount_of_days);
        let random_time_in_seconds =
            TIME_IN_SECONDS_OPTIONS[rand::thread_rng().gen_range(0..TIME_IN_SECONDS_OPTIONS.len())];

        let random_date = Utc::now() - amount_of_days;

        let fake_initial_date = random_date - Duration::seconds(random_time_in_seconds as i64);
        let fake_end_date = random_date + Duration::seconds(random_time_in_seconds as i64);

        Task {
            id: Uuid::new_v4(),
            title: Words(3..5).fake::<Vec<String>>().join(" "),
            description: Words(3..10).fake::<Vec<String>>().join(" "),
            status: TaskStatus::get_random_task_status(),
            user_ref,
            created_at: Utc::now(),
            updated_at: fake_end_date,
            started_at: Some(fake_initial_date),
            finished_at: Some(fake_end_date),
            color: Some(color),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub user_ref: Uuid,
    pub color: Option<String>,
    pub status: Option<String>,
}
