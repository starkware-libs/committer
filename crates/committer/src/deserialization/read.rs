use std::fs;

use crate::deserialization::types::Input;
use crate::deserialization::types::RawInput;
use crate::storage::errors::DeserializationError;

#[cfg(test)]
#[path = "read_test.rs"]
pub mod read_test;

type DeserializationResult<T> = Result<T, DeserializationError>;

pub(crate) fn parse_input(input: String) -> DeserializationResult<Input> {
    serde_json::from_str::<RawInput>(&input)?.try_into()
}

pub fn python_input_parsing_test(temp_input_path: String) -> String {
    let input = fs::read_to_string(temp_input_path).expect("Unable to read given file.");
    match parse_input(input) {
        Ok(actual_input) => create_output_to_python(actual_input),
        Err(DeserializationError::KeyDuplicate(key)) => format!("Test failed. {}", key),
        Err(DeserializationError::ParsingError(serde_error)) => {
            format!("Test failed. {}", serde_error)
        }
    }
}

fn create_output_to_python(actual_input: Input) -> String {
    format!(
        r#"
        {{
        "storage_size": {},
        "address_to_class_hash_size": {},
        "address_to_nonce_size": {},
        "class_hash_to_compiled_class_hash": {},
        "current_contract_state_leaves_size": {},  
        "outer_storage_updates_size": {},
        "tree_height": {}
        }}"#,
        actual_input.storage.len(),
        actual_input.state_diff.address_to_class_hash.len(),
        actual_input.state_diff.address_to_nonce.len(),
        actual_input
            .state_diff
            .class_hash_to_compiled_class_hash
            .len(),
        actual_input.state_diff.current_contract_state_leaves.len(),
        actual_input.state_diff.storage_updates.len(),
        actual_input.tree_height.0
    )
}
