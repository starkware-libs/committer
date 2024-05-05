use committer::{block_committer::types::Input, storage::errors::DeserializationError};

use crate::parse_input::raw_input::RawInput;

#[cfg(test)]
#[path = "read_test.rs"]
pub mod read_test;

type DeserializationResult<T> = Result<T, DeserializationError>;

#[allow(dead_code)]
pub(crate) fn parse_input(input: String) -> DeserializationResult<Input> {
    serde_json::from_str::<RawInput>(&input)?.try_into()
}
