use super::errors::DeserializationError;
use super::types::{Input, RawInput, StarknetStorageKey, StarknetStorageValue, StateDiff};
use crate::deserialization::types::{ContractAddress, ContractState};
use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::filled_node::{ClassHash, Nonce};
use crate::patricia_merkle_tree::types::TreeHeight;
use crate::storage::storage_trait::{StorageKey, StorageValue};
use crate::types::Felt;
use std::collections::HashMap;

impl TryFrom<RawInput> for Input {
    type Error = DeserializationError;
    fn try_from(raw_input: RawInput) -> Result<Self, Self::Error> {
        let mut storage = HashMap::new();
        for entry in raw_input.storage {
            if storage
                .insert(StorageKey(entry.key), StorageValue(entry.value))
                .is_some()
            {
                return Err(DeserializationError::KeyDuplicate("storage".to_string()));
            }
        }

        let mut address_to_class_hash = HashMap::new();
        for entry in raw_input.state_diff.address_to_class_hash {
            if address_to_class_hash
                .insert(
                    ContractAddress(Felt::from_bytes_be_slice(&entry.key)),
                    ClassHash(Felt::from_bytes_be_slice(&entry.value)),
                )
                .is_some()
            {
                return Err(DeserializationError::KeyDuplicate(
                    "address to class hash".to_string(),
                ));
            }
        }

        let mut address_to_nonce = HashMap::new();
        for entry in raw_input.state_diff.address_to_nonce {
            if address_to_nonce
                .insert(
                    ContractAddress(Felt::from_bytes_be_slice(&entry.key)),
                    Nonce(Felt::from_bytes_be_slice(&entry.value)),
                )
                .is_some()
            {
                return Err(DeserializationError::KeyDuplicate(
                    "address to nonce".to_string(),
                ));
            }
        }

        let mut class_hash_to_compiled_class_hash = HashMap::new();
        for entry in raw_input.state_diff.class_hash_to_compiled_class_hash {
            if class_hash_to_compiled_class_hash
                .insert(
                    ClassHash(Felt::from_bytes_be_slice(&entry.key)),
                    ClassHash(Felt::from_bytes_be_slice(&entry.value)),
                )
                .is_some()
            {
                return Err(DeserializationError::KeyDuplicate(
                    "class hash to compiled class hash".to_string(),
                ));
            }
        }

        let mut current_contract_state_leaves = HashMap::new();
        for entry in raw_input.state_diff.current_contract_state_leaves {
            if current_contract_state_leaves
                .insert(
                    ContractAddress(Felt::from_bytes_be_slice(&entry.address)),
                    ContractState {
                        nonce: Nonce(Felt::from_bytes_be_slice(&entry.nonce)),
                        class_hash: ClassHash(Felt::from_bytes_be_slice(&entry.class_hash)),
                        storage_root_hash: HashOutput(Felt::from_bytes_be_slice(
                            &entry.storage_root_hash,
                        )),
                    },
                )
                .is_some()
            {
                return Err(DeserializationError::KeyDuplicate(
                    "current contract state leaves".to_string(),
                ));
            }
        }

        let mut storage_updates = HashMap::new();
        for outer_entry in raw_input.state_diff.storage_updates {
            let inner_map = outer_entry
                .storage_updates
                .iter()
                .map(|inner_entry| {
                    (
                        StarknetStorageKey(Felt::from_bytes_be_slice(&inner_entry.key)),
                        StarknetStorageValue(Felt::from_bytes_be_slice(&inner_entry.value)),
                    )
                })
                .collect();
            if storage_updates
                .insert(
                    ContractAddress(Felt::from_bytes_be_slice(&outer_entry.address)),
                    inner_map,
                )
                .is_some()
            {
                return Err(DeserializationError::KeyDuplicate(
                    "starknet storage updates".to_string(),
                ));
            }
        }

        Ok(Input {
            storage,
            state_diff: StateDiff {
                address_to_class_hash,
                address_to_nonce,
                class_hash_to_compiled_class_hash,
                current_contract_state_leaves,
                storage_updates,
            },
            tree_height: TreeHeight(raw_input.tree_height),
        })
    }
}
