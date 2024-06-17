use serde_json::Value;
use std::collections::HashMap;

use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::node::{ClassHash, CompiledClassHash, Nonce};
use crate::patricia_merkle_tree::node_data::leaf::{ContractState, LeafDataImpl};
use crate::patricia_merkle_tree::types::SubTreeHeight;
use crate::storage::db_object::{DBObject, Deserializable};
use crate::storage::errors::DeserializationError;
use crate::storage::storage_trait::{StorageKey, StoragePrefix, StorageValue};

#[cfg(test)]
#[path = "leaf_serde_test.rs"]
pub mod leaf_serde_test;

impl DBObject for LeafDataImpl {
    /// Serializes the leaf data into a byte vector.
    /// The serialization is done as follows:
    /// - For storage values: serializes the value into a 32-byte vector.
    /// - For compiled class hashes or state tree tuples: creates a  json string
    ///   describing the leaf and cast it into a byte vector.
    fn serialize(&self) -> StorageValue {
        match &self {
            LeafDataImpl::StorageValue(value) => StorageValue(value.to_bytes_be().to_vec()),

            LeafDataImpl::CompiledClassHash(class_hash) => {
                let json_string =
                    format!(r#"{{"compiled_class_hash": "{}"}}"#, class_hash.0.to_hex());
                StorageValue(json_string.into_bytes())
            }

            LeafDataImpl::ContractState(ContractState {
                class_hash,
                storage_root_hash,
                nonce,
            }) => {
                let json_string = format!(
                    r#"{{"contract_hash": "{}", "storage_commitment_tree": {{"root": "{}", "height": {}}}, "nonce": "{}"}}"#,
                    class_hash.0.to_fixed_hex_string(),
                    storage_root_hash.0.to_fixed_hex_string(),
                    SubTreeHeight::ACTUAL_HEIGHT,
                    nonce.0.to_hex(),
                );
                StorageValue(json_string.into_bytes())
            }
        }
    }

    fn get_prefix(&self) -> StoragePrefix {
        match self {
            LeafDataImpl::StorageValue(_) => StoragePrefix::StorageLeaf,
            LeafDataImpl::CompiledClassHash(_) => StoragePrefix::CompiledClassLeaf,
            LeafDataImpl::ContractState { .. } => StoragePrefix::StateTreeLeaf,
        }
    }
}

impl Deserializable for LeafDataImpl {
    fn deserialize(
        _key: &StorageKey,
        value: &StorageValue,
        storage_prefix: &StoragePrefix,
    ) -> Result<Self, DeserializationError> {
        match storage_prefix {
            StoragePrefix::CompiledClassLeaf => {
                let json_str = std::str::from_utf8(&value.0)?;
                let map: HashMap<String, String> = serde_json::from_str(json_str)?;
                let hash_as_hex =
                    map.get("compiled_class_hash")
                        .ok_or(DeserializationError::NonExistingKey(
                            "compiled_class_hash".to_string(),
                        ))?;
                Ok(LeafDataImpl::CompiledClassHash(CompiledClassHash(
                    Felt::from_hex(hash_as_hex)?,
                )))
            }
            StoragePrefix::StorageLeaf => Ok(LeafDataImpl::StorageValue(
                Felt::from_bytes_be_slice(&value.0),
            )),
            StoragePrefix::StateTreeLeaf => {
                let json_str = std::str::from_utf8(&value.0)?;
                let deserialized_map: Value = serde_json::from_str(json_str)?;
                let get_key = |key: &str| {
                    deserialized_map
                        .get(key)
                        .ok_or(DeserializationError::NonExistingKey(key.to_string()))?
                        .as_str()
                        .ok_or(DeserializationError::LeafTypeError)
                };
                let class_hash_as_hex = get_key("contract_hash")?;
                let nonce_as_hex = get_key("nonce")?;
                let root_hash_as_hex = deserialized_map
                    .get("storage_commitment_tree")
                    .ok_or(DeserializationError::NonExistingKey(
                        "storage_commitment_tree".to_string(),
                    ))?
                    .get("root")
                    .ok_or(DeserializationError::NonExistingKey("root".to_string()))?
                    .as_str()
                    .ok_or(DeserializationError::LeafTypeError)?;
                Ok(LeafDataImpl::ContractState(ContractState {
                    nonce: Nonce::from_hex(nonce_as_hex)?,
                    storage_root_hash: HashOutput::from_hex(root_hash_as_hex)?,
                    class_hash: ClassHash::from_hex(class_hash_as_hex)?,
                }))
            }
            _ => Err(DeserializationError::LeafPrefixError(
                storage_prefix.clone(),
            )),
        }
    }
}
