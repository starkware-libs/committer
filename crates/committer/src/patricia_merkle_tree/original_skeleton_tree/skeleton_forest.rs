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
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::patricia_merkle_tree::node_data::leaf::SkeletonContractState;
use crate::patricia_merkle_tree::node_data::leaf::UpdatedSkeletonLeafDataImpl;
use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTreeResult;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::types::TreeHeight;
use crate::patricia_merkle_tree::updated_skeleton_tree::skeleton_forest::UpdatedSkeletonForest;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTree;
use crate::storage::storage_trait::Storage;
use core::marker::PhantomData;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
#[path = "skeleton_forest_test.rs"]
pub mod skeleton_forest_test;

pub(crate) trait OriginalSkeletonForest<L: LeafData + std::clone::Clone> {
    fn create_original_skeleton_forest<S: Storage>(
        storage: S,
        global_tree_root_hash: HashOutput,
        classes_tree_root_hash: HashOutput,
        tree_heights: TreeHeight,
        current_contract_state_leaves: &HashMap<ContractAddress, ContractState>,
        state_diff: &StateDiff,
    ) -> OriginalSkeletonTreeResult<Self>
    where
        Self: std::marker::Sized;

    fn compute_updated_skeleton_forest<U: UpdatedSkeletonTree<L>>(
        &self,
        class_hash_to_compiled_class_hash: HashMap<NodeIndex, L>,
        contracts_to_commit: &HashSet<&ContractAddress>,
        storage_updates: &HashMap<ContractAddress, HashMap<NodeIndex, L>>,
    ) -> OriginalSkeletonTreeResult<UpdatedSkeletonForest<L, U>>;
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct OriginalSkeletonForestImpl<
    L: LeafData + std::clone::Clone,
    T: OriginalSkeletonTree<L>,
> {
    #[allow(dead_code)]
    classes_tree: T,
    #[allow(dead_code)]
    global_state_tree: T,
    #[allow(dead_code)]
    contract_states: HashMap<ContractAddress, T>,
    leaf_data: PhantomData<L>,
}

impl<L: LeafData + std::clone::Clone, T: OriginalSkeletonTree<L>> OriginalSkeletonForest<L>
    for OriginalSkeletonForestImpl<L, T>
{
    fn create_original_skeleton_forest<S: Storage>(
        storage: S,
        global_tree_root_hash: HashOutput,
        classes_tree_root_hash: HashOutput,
        tree_heights: TreeHeight,
        current_contract_state_leaves: &HashMap<ContractAddress, ContractState>,
        state_diff: &StateDiff,
    ) -> OriginalSkeletonTreeResult<Self>
    where
        Self: std::marker::Sized,
    {
        let accessed_addresses = state_diff.accessed_addresses();
        let global_state_tree = Self::create_global_state_tree(
            &accessed_addresses,
            global_tree_root_hash,
            &storage,
            tree_heights,
        )?;
        let contract_states = Self::create_lower_trees_skeleton(
            &accessed_addresses,
            current_contract_state_leaves,
            &state_diff.storage_updates,
            &storage,
            tree_heights,
        )?;
        let classes_tree = Self::create_classes_tree(
            &state_diff.class_hash_to_compiled_class_hash,
            classes_tree_root_hash,
            &storage,
            tree_heights,
        )?;

        Ok(OriginalSkeletonForestImpl::new(
            classes_tree,
            global_state_tree,
            contract_states,
        ))
    }

    fn compute_updated_skeleton_forest<U: UpdatedSkeletonTree<L>>(
        &self,
        _class_hash_to_compiled_class_hash: HashMap<NodeIndex, L>,
        _contracts_to_commit: &HashSet<&ContractAddress>,
        _storage_updates: &HashMap<ContractAddress, HashMap<NodeIndex, L>>,
    ) -> OriginalSkeletonTreeResult<UpdatedSkeletonForest<L, U>> {
        todo!()
    }
}

impl<L: LeafData + std::clone::Clone, T: OriginalSkeletonTree<L>> OriginalSkeletonForestImpl<L, T> {
    pub(crate) fn new(
        classes_tree: T,
        global_state_tree: T,
        contract_states: HashMap<ContractAddress, T>,
    ) -> Self {
        Self {
            classes_tree,
            global_state_tree,
            contract_states,
            leaf_data: PhantomData,
        }
    }

    fn create_global_state_tree<S: Storage>(
        accessed_addresses: &HashSet<&ContractAddress>,
        global_tree_root_hash: HashOutput,
        storage: &S,
        tree_height: TreeHeight,
    ) -> OriginalSkeletonTreeResult<T> {
        let mut sorted_leaf_indices: Vec<NodeIndex> = accessed_addresses
            .iter()
            .map(|address| NodeIndex::from_contract_address(address, &tree_height))
            .collect();
        sorted_leaf_indices.sort();
        T::create_tree(
            storage,
            &sorted_leaf_indices,
            global_tree_root_hash,
            tree_height,
        )
    }

    fn create_lower_trees_skeleton<S: Storage>(
        accessed_addresses: &HashSet<&ContractAddress>,
        current_contract_state_leaves: &HashMap<ContractAddress, ContractState>,
        storage_updates: &HashMap<
            ContractAddress,
            HashMap<StarknetStorageKey, StarknetStorageValue>,
        >,
        storage: &S,
        tree_height: TreeHeight,
    ) -> OriginalSkeletonTreeResult<HashMap<ContractAddress, T>> {
        let mut contract_states = HashMap::new();
        for address in accessed_addresses {
            let mut sorted_leaf_indices: Vec<NodeIndex> = storage_updates
                .get(address)
                .unwrap_or(&HashMap::new())
                .keys()
                .map(|key| NodeIndex::from_starknet_storage_key(key, &tree_height))
                .collect();
            sorted_leaf_indices.sort();
            let contract_state = current_contract_state_leaves
                .get(address)
                .ok_or_else(|| OriginalSkeletonTreeError::LowerTreeCommitmentError(**address))?;
            let original_skeleton = T::create_tree(
                storage,
                &sorted_leaf_indices,
                contract_state.storage_root_hash,
                tree_height,
            )?;
            contract_states.insert(**address, original_skeleton);
        }
        Ok(contract_states)
    }

    fn create_classes_tree<S: Storage>(
        class_hash_to_compiled_class_hash: &HashMap<ClassHash, CompiledClassHash>,
        classes_tree_root_hash: HashOutput,
        storage: &S,
        tree_height: TreeHeight,
    ) -> OriginalSkeletonTreeResult<T> {
        let mut sorted_leaf_indices: Vec<NodeIndex> = class_hash_to_compiled_class_hash
            .keys()
            .map(|class_hash| NodeIndex::from_class_hash(class_hash, &tree_height))
            .collect();
        sorted_leaf_indices.sort();
        T::create_tree(
            storage,
            &sorted_leaf_indices,
            classes_tree_root_hash,
            tree_height,
        )
    }

    #[allow(dead_code)]
    pub(crate) fn compute_updated_skeleton_forest<
        U: UpdatedSkeletonTree<UpdatedSkeletonLeafDataImpl>,
    >(
        &self,
        class_hash_to_compiled_class_hash: HashMap<NodeIndex, L>,
        storage_updates: HashMap<ContractAddress, HashMap<NodeIndex, L>>,
        current_contract_state_leaves: &HashMap<ContractAddress, ContractState>,
        address_to_class_hash: &HashMap<ContractAddress, ClassHash>,
        address_to_nonce: &HashMap<ContractAddress, Nonce>,
        tree_heights: TreeHeight,
    ) -> OriginalSkeletonTreeResult<UpdatedSkeletonForest<UpdatedSkeletonLeafDataImpl, U>> {
        let contract_states = HashMap::new();
        let global_tree_leaves = HashMap::new();
        let classes_tree = self
            .classes_tree
            .compute_updated_skeleton_tree(class_hash_to_compiled_class_hash)?;

        for (address, updates) in storage_updates {
            let original_lower_skeleton = self
                .contract_states
                .get(&address)
                .ok_or_else(|| OriginalSkeletonTreeError::LowerTreeCommitmentError(address))?;
            let updated_lower_skeleton =
                original_lower_skeleton.compute_updated_skeleton_tree(updates)?;

            let new_leaf = get_updated_skeleton_leaf_data_at_address(
                address,
                current_contract_state_leaves,
                address_to_class_hash.get(&address),
                address_to_nonce.get(&address),
                updates.is_empty(),
                updated_lower_skeleton.is_empty(),
            )?;

            contract_states.insert(address, updated_lower_skeleton);
            global_tree_leaves.insert(
                NodeIndex::from_contract_address(&address, &tree_heights),
                new_leaf,
            );
        }
        let global_state_tree = self
            .global_state_tree
            .compute_updated_skeleton_tree(global_tree_leaves);

        Ok(UpdatedSkeletonForest::new(
            classes_tree,
            global_state_tree,
            contract_states,
        ))
    }
}

fn get_updated_skeleton_leaf_data_at_address(
    address: ContractAddress,
    current_contract_state_leaves: &HashMap<ContractAddress, ContractState>,
    new_class_hash: Option<&ClassHash>,
    new_nonce: Option<&Nonce>,
    no_storage_updates: bool,
    is_lower_skeleton_empty: bool,
) -> OriginalSkeletonTreeResult<UpdatedSkeletonLeafDataImpl> {
    let actual_new_nonce = match new_nonce {
        Some(nonce) => *nonce,
        None => {
            current_contract_state_leaves
                .get(&address)
                .ok_or_else(|| OriginalSkeletonTreeError::LowerTreeCommitmentError(address))?
                .nonce
        }
    };

    let actual_new_class_hash = match new_class_hash {
        Some(class_hash) => *class_hash,
        None => {
            current_contract_state_leaves
                .get(&address)
                .ok_or_else(|| OriginalSkeletonTreeError::LowerTreeCommitmentError(address))?
                .class_hash
        }
    };

    let actual_new_storage_root = if no_storage_updates {
        Some(
            current_contract_state_leaves
                .get(&address)
                .ok_or_else(|| OriginalSkeletonTreeError::LowerTreeCommitmentError(address))?
                .storage_root_hash,
        )
    } else if is_lower_skeleton_empty {
        Some(HashOutput(Felt::ZERO))
    } else {
        None
    };

    Ok(UpdatedSkeletonLeafDataImpl::ContractState(
        SkeletonContractState {
            nonce: actual_new_nonce,
            storage_root_hash: actual_new_storage_root,
            class_hash: actual_new_class_hash,
        },
    ))
}
