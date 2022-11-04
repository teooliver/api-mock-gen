use std::fs::File;
use std::io::BufWriter;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use fake::faker::lorem::en::*;
use fake::Fake;
use rand::Rng;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Task {
    id: i32,
    name: String,
    status: TaskStatus,
    user: User,
    started_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    finished_at: DateTime<Utc>,
    color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum TaskStatus {
    Done,
    InProgress,
    NotNedeed,
    ReadyToStart,
}

pub const PROJECT_COLORS: [&str; 10] = [
    "#61e294ff",
    "#7bcdbaff",
    "#9799caff",
    "#bd93d8ff",
    "#b47aeaff",
    "#d3d5d4ff",
    "#a2c5acff",
    "#9db5b2ff",
    "#878e99ff",
    "#7f6a93ff",
];

pub const TIME_IN_SECONDS_OPTIONS: [i32; 7] = [3600, 1800, 5400, 3450, 1600, 1954, 7200];

impl Task {
    fn new_random_todo() -> Task {
        let random_amount_of_days = rand::thread_rng().gen_range(0..=10);
        let amount_of_days = Duration::days(random_amount_of_days);
        let random_time_in_seconds =
            TIME_IN_SECONDS_OPTIONS[rand::thread_rng().gen_range(0..TIME_IN_SECONDS_OPTIONS.len())];

        let random_date = Utc::now() - amount_of_days;

        let fake_initial_date = random_date - Duration::seconds(random_time_in_seconds as i64);
        let fake_end_date = random_date + Duration::seconds(random_time_in_seconds as i64);

        Task {
            id: 100,
            name: Words(3..5).fake::<Vec<String>>().join(" "),
            status: TaskStatus::Done,
            user: User {
                id: "100".to_string(),
                name: fake::faker::name::en::FirstName()
                    .fake::<String>()
                    .to_string(),
                email: fake::faker::name::en::FirstName()
                    .fake::<String>()
                    .to_string(),
            },
            started_at: fake_initial_date,
            updated_at: fake_end_date,
            finished_at: fake_end_date,
            color: rand::thread_rng()
                .gen_range(0..(PROJECT_COLORS.len() - 1))
                .to_string(),
        }
    }

    fn get_random_task_status() -> TaskStatus {
        // TODO: use this when stable so we dont have to
        // hard code the enum length in the `gen_range`, that way we can
        // avoid breaking functionality with the enum changes.
        //
        // https://github.com/rust-lang/rust/issues/73662
        // let enum_length = mem::variant_count::<TaskStatus>();

        match rand::thread_rng().gen_range(0..=4) {
            0 => TaskStatus::Done,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::NotNedeed,
            3 => TaskStatus::ReadyToStart,
            _ => TaskStatus::ReadyToStart,
        }
    }
}

fn main() {
    // let input_path = std::env::args().nth(1).unwrap();
    // let output_path = std::env::args().nth(2).unwrap();

    let todos = generate_todo_list_struc(10);
    let random_enum_value = Task::get_random_task_status();
    // println!("{:?}", random_enum_value);
    generate_json_db(&todos, "tasks_json_db.json".to_string());
}

fn generate_json_db(tasks: &Vec<Task>, output_path: String) {
    let mut writer = BufWriter::new(File::create("tasks_json_db.json").unwrap());
    serde_json::to_writer_pretty(&mut writer, &tasks).unwrap();
}

fn generate_todo_list_struc(amount: u8) -> Vec<Task> {
    let mut todos: Vec<Task> = vec![];

    for _n in 1..=amount {
        todos.push(Task::new_random_todo());
    }

    todos
}
