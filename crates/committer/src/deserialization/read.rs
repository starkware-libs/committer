use crate::deserialization::types::Input;
use crate::deserialization::types::RawInput;

use crate::deserialization::errors::DeserializationError;
#[allow(dead_code)]
type DeserializationResult<T> = Result<T, DeserializationError>;
#[allow(dead_code)]
pub(crate) fn parse_input(input: String) -> DeserializationResult<Input> {
    let raw_input: RawInput = serde_json::from_str(&input)?;
    raw_input.try_into()
}

#[cfg(test)]
#[allow(dead_code)]
fn test_input_parsing() {
    todo!()
}
