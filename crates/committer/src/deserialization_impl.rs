use std::collections::HashMap;

use crate::deserialization_types::{
    ActualInput, ActualStateDiff, ContractState, DeserializedInput,
};
use crate::patricia_merkle_tree::filled_node::{ClassHash, Nonce};
use starknet_types_core::felt::Felt;

#[allow(dead_code)]
impl DeserializedInput {
    pub fn actual_input(self) -> ActualInput {
        let mut storage = HashMap::new();
        for entry in self.storage.into_iter() {
            assert!(storage.insert(entry.key, entry.value).is_none())
        }

        let mut address_to_class_hash = HashMap::new();
        for entry in self.state_diff.address_to_class_hash {
            assert!(address_to_class_hash
                .insert(
                    Felt::from_bytes_be_slice(&entry.key),
                    ClassHash(Felt::from_bytes_be_slice(&entry.value))
                )
                .is_none());
        }

        let mut address_to_nonce = HashMap::new();
        for entry in self.state_diff.address_to_nonce {
            assert!(address_to_nonce
                .insert(
                    Felt::from_bytes_be_slice(&entry.key),
                    Nonce(Felt::from_bytes_be_slice(&entry.value))
                )
                .is_none());
        }

        let mut class_hash_to_compiled_class_hash = HashMap::new();
        for entry in self.state_diff.class_hash_to_compiled_class_hash {
            assert!(class_hash_to_compiled_class_hash
                .insert(
                    ClassHash(Felt::from_bytes_be_slice(&entry.key)),
                    ClassHash(Felt::from_bytes_be_slice(&entry.value))
                )
                .is_none());
        }

        let mut current_contract_state_leaves = HashMap::new();
        for entry in self.state_diff.current_contract_state_leaves {
            assert!(current_contract_state_leaves
                .insert(
                    Felt::from_bytes_be_slice(&entry.address),
                    ContractState {
                        nonce: Nonce(Felt::from_bytes_be_slice(&entry.nonce)),
                        class_hash: ClassHash(Felt::from_bytes_be_slice(&entry.class_hash)),
                        storage_root_hash: Felt::from_bytes_be_slice(&entry.storage_root_hash)
                    }
                )
                .is_none());
        }

        let mut storage_updates: HashMap<Felt, HashMap<Felt, Felt>> = HashMap::new();
        for outer_entry in self.state_diff.storage_updates {
            let tmp_map: HashMap<Felt, Felt> = outer_entry
                .storage_updates
                .iter()
                .map(|inner_entry| {
                    (
                        Felt::from_bytes_be_slice(&inner_entry.key),
                        Felt::from_bytes_be_slice(&inner_entry.value),
                    )
                })
                .collect();
            storage_updates.insert(Felt::from_bytes_be_slice(&outer_entry.address), tmp_map);
        }

        ActualInput {
            storage,
            state_diff: ActualStateDiff {
                address_to_class_hash,
                address_to_nonce,
                class_hash_to_compiled_class_hash,
                current_contract_state_leaves,
                storage_updates,
            },
        }
    }
}
