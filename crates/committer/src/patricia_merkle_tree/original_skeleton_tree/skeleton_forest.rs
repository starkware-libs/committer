use crate::block_committer::input::ContractAddress;
use crate::block_committer::input::StarknetStorageKey;
use crate::block_committer::input::StarknetStorageValue;
use crate::block_committer::input::StateDiff;
use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::node::ClassHash;
use crate::patricia_merkle_tree::filled_tree::node::CompiledClassHash;
use crate::patricia_merkle_tree::filled_tree::node::Nonce;
use crate::patricia_merkle_tree::node_data::leaf::ContractState;
use crate::patricia_merkle_tree::node_data::leaf::LeafModifications;
use crate::patricia_merkle_tree::node_data::leaf::SkeletonLeaf;
use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTreeResult;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::types::TreeHeight;
use crate::patricia_merkle_tree::updated_skeleton_tree::errors::UpdatedSkeletonTreeError;
use crate::patricia_merkle_tree::updated_skeleton_tree::skeleton_forest::UpdatedSkeletonForest;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTree;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTreeResult;
use crate::storage::storage_trait::Storage;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
#[path = "skeleton_forest_test.rs"]
pub mod skeleton_forest_test;

pub(crate) trait OriginalSkeletonForest {
    fn create_original_skeleton_forest(
        storage: impl Storage,
        contracts_trie_root_hash: HashOutput,
        classes_trie_root_hash: HashOutput,
        tree_heights: TreeHeight,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        state_diff: &StateDiff,
    ) -> OriginalSkeletonTreeResult<Self>
    where
        Self: std::marker::Sized;

    fn compute_updated_skeleton_forest<U: UpdatedSkeletonTree>(
        &self,
        class_hash_leaf_modifications: &LeafModifications<SkeletonLeaf>,
        storage_updates: &HashMap<ContractAddress, LeafModifications<SkeletonLeaf>>,
        current_contract_state_leaves: &HashMap<ContractAddress, ContractState>,
        address_to_class_hash: &HashMap<ContractAddress, ClassHash>,
        address_to_nonce: &HashMap<ContractAddress, Nonce>,
        tree_heights: TreeHeight,
    ) -> UpdatedSkeletonTreeResult<UpdatedSkeletonForest<U>>;
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct OriginalSkeletonForestImpl<T: OriginalSkeletonTree> {
    #[allow(dead_code)]
    classes_trie: T,
    #[allow(dead_code)]
    contracts_trie: T,
    #[allow(dead_code)]
    storage_tries: HashMap<ContractAddress, T>,
}

impl<T: OriginalSkeletonTree> OriginalSkeletonForest for OriginalSkeletonForestImpl<T> {
    fn create_original_skeleton_forest(
        storage: impl Storage,
        contracts_trie_root_hash: HashOutput,
        classes_trie_root_hash: HashOutput,
        tree_heights: TreeHeight,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        state_diff: &StateDiff,
    ) -> OriginalSkeletonTreeResult<Self>
    where
        Self: std::marker::Sized,
    {
        let accessed_addresses = state_diff.accessed_addresses();
        let global_state_tree = Self::create_contracts_trie(
            &accessed_addresses,
            contracts_trie_root_hash,
            &storage,
            tree_heights,
        )?;
        let contract_states = Self::create_storage_tries(
            &accessed_addresses,
            current_contracts_trie_leaves,
            &state_diff.storage_updates,
            &storage,
            tree_heights,
        )?;
        let classes_tree = Self::create_classes_trie(
            &state_diff.class_hash_to_compiled_class_hash,
            classes_trie_root_hash,
            &storage,
            tree_heights,
        )?;

        Ok(OriginalSkeletonForestImpl::new(
            classes_tree,
            global_state_tree,
            contract_states,
        ))
    }

    fn compute_updated_skeleton_forest<U: UpdatedSkeletonTree>(
        &self,
        class_hash_leaf_modifications: &LeafModifications<SkeletonLeaf>,
        storage_updates: &HashMap<ContractAddress, LeafModifications<SkeletonLeaf>>,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        address_to_class_hash: &HashMap<ContractAddress, ClassHash>,
        address_to_nonce: &HashMap<ContractAddress, Nonce>,
        tree_heights: TreeHeight,
    ) -> UpdatedSkeletonTreeResult<UpdatedSkeletonForest<U>> {
        // Classes trie.
        let classes_trie = U::create(&self.classes_trie, class_hash_leaf_modifications)?;

        // Storage tries.
        let mut contracts_trie_leaves = HashMap::new();
        let mut storage_tries = HashMap::new();

        for (address, updates) in storage_updates {
            let original_storage_trie = self
                .storage_tries
                .get(address)
                .ok_or(UpdatedSkeletonTreeError::LowerTreeCommitmentError(*address))?;

            let updated_storage_trie = U::create(original_storage_trie, updates)?;
            let storage_trie_becomes_empty = updated_storage_trie.is_empty();

            storage_tries.insert(*address, updated_storage_trie);

            let current_leaf = current_contracts_trie_leaves
                .get(address)
                .ok_or(UpdatedSkeletonTreeError::LowerTreeCommitmentError(*address))?;

            let skeleton_leaf = Self::updated_contract_skeleton_leaf(
                address_to_nonce.get(address),
                address_to_class_hash.get(address),
                current_leaf,
                storage_trie_becomes_empty,
            );
            contracts_trie_leaves.insert(
                NodeIndex::from_contract_address(address, &tree_heights),
                skeleton_leaf,
            );
        }

        // Contracts trie.
        let contracts_trie = U::create(&self.contracts_trie, &contracts_trie_leaves)?;

        Ok(UpdatedSkeletonForest {
            classes_trie,
            contracts_trie,
            storage_tries,
        })
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
        tree_height: TreeHeight,
    ) -> OriginalSkeletonTreeResult<T> {
        let mut sorted_leaf_indices: Vec<NodeIndex> = accessed_addresses
            .iter()
            .map(|address| NodeIndex::from_contract_address(address, &tree_height))
            .collect();
        sorted_leaf_indices.sort();
        T::create(
            storage,
            &sorted_leaf_indices,
            contracts_trie_root_hash,
            tree_height,
        )
    }

    fn create_storage_tries(
        accessed_addresses: &HashSet<&ContractAddress>,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        storage_updates: &HashMap<
            ContractAddress,
            HashMap<StarknetStorageKey, StarknetStorageValue>,
        >,
        storage: &impl Storage,
        tree_height: TreeHeight,
    ) -> OriginalSkeletonTreeResult<HashMap<ContractAddress, T>> {
        let mut storage_tries = HashMap::new();
        for address in accessed_addresses {
            let mut sorted_leaf_indices: Vec<NodeIndex> = storage_updates
                .get(address)
                .unwrap_or(&HashMap::new())
                .keys()
                .map(|key| NodeIndex::from_starknet_storage_key(key, &tree_height))
                .collect();
            sorted_leaf_indices.sort();
            let contract_state = current_contracts_trie_leaves.get(address).ok_or(
                OriginalSkeletonTreeError::LowerTreeCommitmentError(**address),
            )?;
            let original_skeleton = T::create(
                storage,
                &sorted_leaf_indices,
                contract_state.storage_root_hash,
                tree_height,
            )?;
            storage_tries.insert(**address, original_skeleton);
        }
        Ok(storage_tries)
    }

    fn create_classes_trie(
        class_hash_to_compiled_class_hash: &HashMap<ClassHash, CompiledClassHash>,
        classes_trie_root_hash: HashOutput,
        storage: &impl Storage,
        tree_height: TreeHeight,
    ) -> OriginalSkeletonTreeResult<T> {
        let mut sorted_leaf_indices: Vec<NodeIndex> = class_hash_to_compiled_class_hash
            .keys()
            .map(|class_hash| NodeIndex::from_class_hash(class_hash, &tree_height))
            .collect();
        sorted_leaf_indices.sort();
        T::create(
            storage,
            &sorted_leaf_indices,
            classes_trie_root_hash,
            tree_height,
        )
    }

    /// Given the previous contract state, whether the contract's storage has become empty or not,
    /// optional new nonce & new class hash, the function creates a skeleton leaf.
    fn updated_contract_skeleton_leaf(
        new_nonce: Option<&Nonce>,
        new_class_hash: Option<&ClassHash>,
        previous_state: &ContractState,
        storage_becomes_empty: bool,
    ) -> SkeletonLeaf {
        let actual_new_nonce = new_nonce.unwrap_or(&previous_state.nonce);
        let actual_new_class_hash = new_class_hash.unwrap_or(&previous_state.class_hash);
        if storage_becomes_empty
            && actual_new_nonce.0 == Felt::ZERO
            && actual_new_class_hash.0 == Felt::ZERO
        {
            SkeletonLeaf::Zero
        } else {
            SkeletonLeaf::NonZero
        }
    }
}
