use std::{fs::File, io};

use committer::{block_committer::input::Input, storage::errors::DeserializationError};
use serde::{Deserialize, Serialize};

use crate::parse_input::raw_input::RawInput;

#[cfg(test)]
#[path = "read_test.rs"]
pub mod read_test;

type DeserializationResult<T> = Result<T, DeserializationError>;

pub fn parse_input(input: &str) -> DeserializationResult<Input> {
    serde_json::from_str::<RawInput>(input)?.try_into()
}

pub fn read_from_stdin() -> String {
    io::read_to_string(io::stdin()).expect("Failed to read from stdin.")
}

pub fn load_from_stdin<T: for<'a> Deserialize<'a>>() -> T {
    let stdin = read_from_stdin();
    serde_json::from_str(&stdin).expect("Failed to load from stdin")
}

pub fn write_to_file<T: Serialize>(file_path: &str, object: &T) {
    let file = File::create(file_path).expect("Failed to create file");
    serde_json::to_writer(file, object).expect("Failed to serialize");
}
