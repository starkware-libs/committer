use crate::block_committer::types::ContractAddress;
use crate::block_committer::types::Input;
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTreeResult;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::storage_trait::Storage;
use core::marker::PhantomData;
use std::collections::HashMap;

pub(crate) struct OriginalSkeletonForest<
    L: LeafData + std::clone::Clone,
    T: OriginalSkeletonTree<L>,
> {
    // TODO(Nimrod): Add compiled class tree.
    // TODO(Nimrod): Add global state tree.
    #[allow(dead_code)]
    contract_states: HashMap<ContractAddress, T>,
    leaf_data: PhantomData<L>,
}

#[allow(dead_code)]
pub(crate) fn create_original_skeleton_forest<
    L: LeafData + std::clone::Clone,
    T: OriginalSkeletonTree<L>,
    S: Storage,
>(
    input: Input,
) -> OriginalSkeletonTreeResult<OriginalSkeletonForest<L, T>> {
    let actual_storage = S::from(input.storage);
    let mut contract_states = HashMap::new();
    let accessed_addresses = input.state_diff.accessed_addresses();
    for address in accessed_addresses {
        let mut sorted_leaf_indices: Vec<NodeIndex> = input
            .state_diff
            .storage_updates
            .get(&address)
            .unwrap_or(&HashMap::new())
            .keys()
            .map(|key| NodeIndex::from(*key).as_full_index(&input.tree_height))
            .collect();
        sorted_leaf_indices.sort();
        let cur_contract_state = input
            .state_diff
            .current_contract_state_leaves
            .get(&address)
            .ok_or_else(|| OriginalSkeletonTreeError::LowerTreeCommitmentError(address))?;
        let cur_original_skeleton = T::create_tree(
            &actual_storage,
            &sorted_leaf_indices,
            cur_contract_state.storage_root_hash,
            input.tree_height,
        )?;
        contract_states.insert(address, cur_original_skeleton);
    }
    Ok(OriginalSkeletonForest {
        contract_states,
        leaf_data: PhantomData,
    })
}
