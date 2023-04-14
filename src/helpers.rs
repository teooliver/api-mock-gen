use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;

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
