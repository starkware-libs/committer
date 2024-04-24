use serde::{Deserialize, Serialize};

use crate::patricia_merkle_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::node_serde::SerializeNode;

/// Temporary struct to serialize the leaf CompiledClass.
/// Required to comply to existing storage layout.
#[derive(Serialize, Deserialize)]
pub(crate) struct LeafCompiledClassToSerialize {
    pub(crate) compiled_class_hash: Felt,
}
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::types::Felt;

impl LeafData {
    /// Serializes the leaf data into a byte vector.
    /// The serialization is done as follows:
    /// - For storage values: serializes the value into a 32-byte vector.
    /// - For compiled class hashes or state tree tuples: creates a  json string
    ///   describing the leaf and cast it into a byte vector.
    pub(crate) fn serialize(&self) -> Result<SerializeNode, FilledTreeError> {
        match &self {
            LeafData::StorageValue(value) => {
                Ok(SerializeNode::StorageLeaf(value.as_bytes().to_vec()))
            }

            LeafData::CompiledClassHash(class_hash) => {
                // Create a temporary object to serialize the leaf into a JSON.
                let temp_object_to_json = LeafCompiledClassToSerialize {
                    compiled_class_hash: class_hash.0,
                };

                // Serialize the leaf into a JSON.
                let json = serde_json::to_string(&temp_object_to_json)?;

                // Serialize the json into a byte vector.
                Ok(SerializeNode::CompiledClassLeaf(
                    json.into_bytes().to_owned(),
                ))
            }

            LeafData::StateTreeTuple { .. } => {
                todo!("implement.");
            }
        }
    }
}
