use committer::felt::Felt;
use committer::hash::hash_trait::HashOutput;
use committer::hash::hash_trait::{HashFunction, HashInputPair};
use committer::hash::pedersen::PedersenHashFunction;
use committer::patricia_merkle_tree::filled_tree::node::{ClassHash, FilledNode, Nonce};
use committer::patricia_merkle_tree::node_data::inner_node::{BinaryData, EdgeData, NodeData};
use committer::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use committer::storage::serde_trait::Serializable;
use std::collections::HashMap;
use thiserror;

// Enum representing different Python tests.
pub(crate) enum PythonTest {
    ExampleTest,
    FeltSerialize,
    HashFunction,
    BinarySerialize,
    NodeKey,
}

/// Error type for PythonTest enum.
#[derive(Debug, thiserror::Error)]
pub(crate) enum PythonTestError {
    #[error("Unknown test name: {0}")]
    UnknownTestName(String),
    #[error("Failed to parse input: {0}")]
    ParseInputError(#[from] serde_json::Error),
    #[error("Failed to parse input: {0} to int")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// Implements conversion from a string to a `PythonTest`.
impl TryFrom<String> for PythonTest {
    type Error = PythonTestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "example_test" => Ok(Self::ExampleTest),
            "felt_serialize_test" => Ok(Self::FeltSerialize),
            "hash_function_test" => Ok(Self::HashFunction),
            "binary_serialize_test" => Ok(Self::BinarySerialize),
            "node_db_key_test" => Ok(Self::NodeKey),
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
            Self::FeltSerialize => {
                let felt = input.parse::<u128>()?;
                Ok(felt_serialize_test(felt))
            }
            Self::HashFunction => {
                let hash_input: HashMap<String, u128> = serde_json::from_str(input)?;
                let x = hash_input
                    .get("x")
                    .expect("Failed to get value for key 'x'");
                let y = hash_input
                    .get("y")
                    .expect("Failed to get value for key 'y'");
                Ok(test_hash_function(*x, *y))
            }
            Self::BinarySerialize => {
                let binary_input: HashMap<String, u128> = serde_json::from_str(input)?;
                let left = binary_input
                    .get("left")
                    .expect("Failed to get value for key 'left'");
                let right = binary_input
                    .get("right")
                    .expect("Failed to get value for key 'right'");
                Ok(test_binary_serialize_test(*left, *right))
            }
            Self::NodeKey => Ok(test_node_db_key()),
        }
    }
}

pub(crate) fn example_test(test_args: HashMap<String, String>) -> String {
    let x = test_args.get("x").expect("Failed to get value for key 'x'");
    let y = test_args.get("y").expect("Failed to get value for key 'y'");
    format!("Calling example test with args: x: {}, y: {}", x, y)
}

/// Serializes a Felt into a string.
pub(crate) fn felt_serialize_test(felt: u128) -> String {
    let bytes = Felt::from(felt).as_bytes().to_vec();
    serde_json::to_string(&bytes)
        .unwrap_or_else(|error| panic!("Failed to serialize felt: {}", error))
}

pub(crate) fn test_hash_function(x: u128, y: u128) -> String {
    let x_felt = Felt::from(x);
    let y_felt = Felt::from(y);
    let hash_result = PedersenHashFunction::compute_hash(HashInputPair(x_felt, y_felt)).0;
    serde_json::to_string(&hash_result)
        .unwrap_or_else(|error| panic!("Failed to serialize hash result: {}", error))
}

/// Serializes binary data into a JSON string.
/// # Arguments
///
/// * `left` - The left 128-bit integer used to create binary data.
/// * `right` - The right 128-bit integer used to create binary data.
///
/// # Returns
///
/// A JSON string representing the serialized binary data.
pub(crate) fn test_binary_serialize_test(left: u128, right: u128) -> String {
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();

    let binary_data = BinaryData {
        left_hash: HashOutput(Felt::from(left)),
        right_hash: HashOutput(Felt::from(right)),
    };

    // Create a filled node with binary data and zero hash.
    let filled_node = FilledNode {
        data: NodeData::Binary(binary_data),
        hash: HashOutput(Felt::ZERO),
    };

    // Serialize the binary node and insert it into the map under the key "value".
    let value = filled_node
        .serialize()
        .unwrap_or_else(|error| panic!("Failed to serialize binary data: {}", error));
    map.insert("value".to_string(), value.0);

    // Serialize the map to a JSON string and handle serialization errors.
    serde_json::to_string(&map)
        .unwrap_or_else(|error| panic!("Failed to serialize binary fact: {}", error))
}

/// Creates and serializes storage keys for different node types.
///
/// This function generates and serializes storage keys for various node types, including binary nodes,
/// edge nodes, storage leaf nodes, state tree leaf nodes, and compiled class leaf nodes. The resulting
/// keys are stored in a `HashMap` and serialized into a JSON string.
///
/// # Returns
///
/// A JSON string representing the serialized storage keys for different node types.
///
pub(crate) fn test_node_db_key() -> String {
    let zero: u128 = 0;

    // Generate keys for different node types.
    let hash = HashOutput(Felt::from(zero));

    let binary_node_key = FilledNode {
        data: NodeData::Binary(BinaryData {
            left_hash: hash,
            right_hash: hash,
        }),
        hash,
    }
    .db_key()
    .0;

    let edge_node_key = FilledNode {
        data: NodeData::Edge(EdgeData {
            bottom_hash: hash,
            path_to_bottom: Default::default(),
        }),
        hash,
    }
    .db_key()
    .0;

    let storage_leaf_key = FilledNode {
        data: NodeData::Leaf(LeafDataImpl::StorageValue(Felt::from(zero))),
        hash,
    }
    .db_key()
    .0;

    let state_tree_leaf_key = FilledNode {
        data: NodeData::Leaf(LeafDataImpl::StateTreeTuple {
            class_hash: ClassHash(Felt::from(zero)),
            contract_state_root_hash: Felt::from(zero),
            nonce: Nonce(Felt::from(zero)),
        }),
        hash,
    }
    .db_key()
    .0;

    let compiled_class_leaf_key = FilledNode {
        data: NodeData::Leaf(LeafDataImpl::CompiledClassHash(ClassHash(Felt::from(zero)))),
        hash,
    }
    .db_key()
    .0;

    // Store keys in a HashMap.
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();

    map.insert("binary_node_key".to_string(), binary_node_key);
    map.insert("edge_node_key".to_string(), edge_node_key);
    map.insert("storage_leaf_key".to_string(), storage_leaf_key);
    map.insert("state_tree_leaf_key".to_string(), state_tree_leaf_key);
    map.insert(
        "compiled_class_leaf_key".to_string(),
        compiled_class_leaf_key,
    );

    // Serialize the map to a JSON string and handle serialization errors.
    serde_json::to_string(&map)
        .unwrap_or_else(|error| panic!("Failed to serialize storage prefix: {}", error))
}
