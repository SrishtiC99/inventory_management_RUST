use std::{fs::{File, OpenOptions}, io::{Read, Write, self}};
use serde::Serialize;

pub fn read_file(filename: String) -> String {
    let mut file = File::open(filename).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");
    contents
}

pub fn save_file<T>(filename: String, list: &[T]) ->  io::Result<()>
where
    T: Serialize,
{
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;

    let json_data = serde_json::to_string_pretty(list)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}