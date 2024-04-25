use committer::storage::{
    errors::DeserializationError,
    storage_trait::{StorageKey, StorageValue},
};
use std::{collections::HashMap, io};
use thiserror;

use crate::deserialization::{read::parse_input, types::Input};

// Enum representing different Python tests.
pub(crate) enum PythonTest {
    ExampleTest,
    InputParsing,
}

/// Error type for PythonTest enum.
#[derive(Debug, thiserror::Error)]
pub(crate) enum PythonTestError {
    #[error("Unknown test name: {0}")]
    UnknownTestName(String),
    #[error("Failed to parse input: {0}")]
    ParseInputError(#[from] serde_json::Error),
    #[error("Test failed. {0}")]
    DeserializationTestFailure(#[from] DeserializationError),
    #[error("Failed to read from stdin.")]
    StdinReadError(#[from] io::Error),
}

/// Implements conversion from a string to a `PythonTest`.
impl TryFrom<String> for PythonTest {
    type Error = PythonTestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "example_test" => Ok(Self::ExampleTest),
            "input_parsing" => Ok(Self::InputParsing),
            _ => Err(PythonTestError::UnknownTestName(value)),
        }
    }
}

impl PythonTest {
    /// Runs the test with the given arguments.
    pub(crate) fn run(&self, input: &str) -> Result<String, PythonTestError> {
        match self {
            Self::ExampleTest => {
                let example_input: HashMap<String, String> = serde_json::from_str(input)?;
                Ok(example_test(example_input))
            }
            Self::InputParsing => parse_input_test(),
        }
    }
}

pub(crate) fn example_test(test_args: HashMap<String, String>) -> String {
    let x = test_args.get("x").expect("Failed to get value for key 'x'");
    let y = test_args.get("y").expect("Failed to get value for key 'y'");

    format!("Calling example test with args: x: {}, y: {}", x, y)
}

pub(crate) fn parse_input_test() -> Result<String, PythonTestError> {
    let input = io::read_to_string(io::stdin())?;
    Ok(create_output_to_python(parse_input(input)?))
}

fn create_output_to_python(actual_input: Input) -> String {
    let (storage_keys_hash, storage_values_hash) = hash_storage(&actual_input.storage);
    format!(
        r#"
        {{
        "storage_size": {},
        "address_to_class_hash_size": {},
        "address_to_nonce_size": {},
        "class_hash_to_compiled_class_hash": {},
        "current_contract_state_leaves_size": {},
        "outer_storage_updates_size": {},
        "tree_height": {},
        "storage_keys_hash": {:?},
        "storage_values_hash": {:?}
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
        actual_input.tree_height.0,
        storage_keys_hash,
        storage_values_hash
    )
}

fn hash_storage(storage: &HashMap<StorageKey, StorageValue>) -> (Vec<u8>, Vec<u8>) {
    let mut keys_hash = vec![0; 32];
    let mut values_hash = vec![0; 32];
    for (key, value) in storage {
        keys_hash = xor_hash(&keys_hash, &key.0);
        values_hash = xor_hash(&values_hash, &value.0);
    }
    (keys_hash, values_hash)
}

fn xor_hash(x: &[u8], y: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    for (a, b) in x.iter().zip(y.iter()) {
        res.push(a ^ b)
    }
    res
}
