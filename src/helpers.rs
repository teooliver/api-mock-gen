use serde::Serialize;
use std::fs::{self, File};
use std::io::BufWriter;

use crate::db::AppData;

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

pub fn generate_json_file<T: Serialize>(input: &Vec<T>, output_path: String) {
    // TODO: Instead of output path, we should write to the same path but grab the
    // key we want to insert each "collection" to.

    let mut writer = BufWriter::new(File::create(output_path).unwrap());
    serde_json::to_writer_pretty(&mut writer, &input).unwrap();
}

pub fn write_struct_to_json<T>(input: T, output_path: String) {
    todo!()
}

fn generate_json_files(data: &AppData) {
    // TODO: Deal with possible errors from create_dir
    let _ = fs::create_dir("./mocked_db");
    // QUESTION: Should the "collections" be created all in on json file,
    // or should we keep them separate
    // TODO: Serve those json files in routes, just as an example on how to
    // serve files on axum. Could be also usefull as a way of grabing all info all at once
    // in the case we have one json file with all collections
    generate_json_file(&data.tasks, "mocked_db/tasks_json_db.json".to_string());
    generate_json_file(&data.users, "mocked_db/users_json_db.json".to_string());
}
