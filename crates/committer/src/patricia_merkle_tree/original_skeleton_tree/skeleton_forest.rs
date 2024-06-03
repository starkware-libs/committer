use crate::block_committer::input::ContractAddress;
use crate::block_committer::input::StarknetStorageKey;
use crate::block_committer::input::StarknetStorageValue;
use crate::block_committer::input::StateDiff;
use crate::forest_errors::ForestError;
use crate::forest_errors::ForestResult;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::node::ClassHash;
use crate::patricia_merkle_tree::filled_tree::node::CompiledClassHash;
use crate::patricia_merkle_tree::node_data::leaf::ContractState;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::storage_trait::Storage;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
#[path = "skeleton_forest_test.rs"]
pub mod skeleton_forest_test;

pub(crate) trait OriginalSkeletonForest {
    fn create(
        storage: impl Storage,
        contracts_trie_root_hash: HashOutput,
        classes_trie_root_hash: HashOutput,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        state_diff: &StateDiff,
    ) -> ForestResult<Self>
    where
        Self: std::marker::Sized;
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct OriginalSkeletonForestImpl<T: OriginalSkeletonTree> {
    pub(crate) classes_trie: T,
    pub(crate) contracts_trie: T,
    pub(crate) storage_tries: HashMap<ContractAddress, T>,
}

impl<T: OriginalSkeletonTree> OriginalSkeletonForest for OriginalSkeletonForestImpl<T> {
    fn create(
        storage: impl Storage,
        contracts_trie_root_hash: HashOutput,
        classes_trie_root_hash: HashOutput,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        state_diff: &StateDiff,
    ) -> ForestResult<Self>
    where
        Self: std::marker::Sized,
    {
        let accessed_addresses = state_diff.accessed_addresses();
        let global_state_tree =
            Self::create_contracts_trie(&accessed_addresses, contracts_trie_root_hash, &storage)?;
        let contract_states = Self::create_storage_tries(
            &accessed_addresses,
            current_contracts_trie_leaves,
            &state_diff.storage_updates,
            &storage,
        )?;
        let classes_tree = Self::create_classes_trie(
            &state_diff.class_hash_to_compiled_class_hash,
            classes_trie_root_hash,
            &storage,
        )?;

        Ok(OriginalSkeletonForestImpl::new(
            classes_tree,
            global_state_tree,
            contract_states,
        ))
    }
}

impl<T: OriginalSkeletonTree> OriginalSkeletonForestImpl<T> {
    pub(crate) fn new(
        classes_trie: T,
        contracts_trie: T,
        storage_tries: HashMap<ContractAddress, T>,
    ) -> Self {
        Self {
            classes_trie,
            contracts_trie,
            storage_tries,
        }
    }

    fn create_contracts_trie(
        accessed_addresses: &HashSet<&ContractAddress>,
        contracts_trie_root_hash: HashOutput,
        storage: &impl Storage,
    ) -> ForestResult<T> {
        let mut sorted_leaf_indices: Vec<NodeIndex> = accessed_addresses
            .iter()
            .map(|address| NodeIndex::from_contract_address(address))
            .collect();
        sorted_leaf_indices.sort();
        Ok(T::create(
            storage,
            &sorted_leaf_indices,
            contracts_trie_root_hash,
        )?)
    }

    fn create_storage_tries(
        accessed_addresses: &HashSet<&ContractAddress>,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        storage_updates: &HashMap<
            ContractAddress,
            HashMap<StarknetStorageKey, StarknetStorageValue>,
        >,
        storage: &impl Storage,
    ) -> ForestResult<HashMap<ContractAddress, T>> {
        let mut storage_tries = HashMap::new();
        for address in accessed_addresses {
            let mut sorted_leaf_indices: Vec<NodeIndex> = storage_updates
                .get(address)
                .unwrap_or(&HashMap::new())
                .keys()
                .map(NodeIndex::from_starknet_storage_key)
                .collect();
            sorted_leaf_indices.sort();
            let contract_state = current_contracts_trie_leaves
                .get(address)
                .ok_or(ForestError::MissingContractCurrentState(**address))?;
            let original_skeleton = T::create(
                storage,
                &sorted_leaf_indices,
                contract_state.storage_root_hash,
            )?;
            storage_tries.insert(**address, original_skeleton);
        }
        Ok(storage_tries)
    }

    fn create_classes_trie(
        class_hash_to_compiled_class_hash: &HashMap<ClassHash, CompiledClassHash>,
        classes_trie_root_hash: HashOutput,
        storage: &impl Storage,
    ) -> ForestResult<T> {
        let mut sorted_leaf_indices: Vec<NodeIndex> = class_hash_to_compiled_class_hash
            .keys()
            .map(NodeIndex::from_class_hash)
            .collect();
        sorted_leaf_indices.sort();
        Ok(T::create(
            storage,
            &sorted_leaf_indices,
            classes_trie_root_hash,
        )?)
    }
}
