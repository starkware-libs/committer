use std::collections::HashMap;

use crate::block_committer::errors::BlockCommitmentError;
use crate::block_committer::types::ContractAddress;
use crate::block_committer::types::Input;
use crate::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::storage_trait::Storage;

type BlockCommitmentResult<T> = Result<T, BlockCommitmentError>;

#[allow(dead_code)]
pub(crate) struct OriginalSkeletonForest<S: OriginalSkeletonTree<LeafDataImpl>> {
    // TODO(Nimrod): Add compiled class tree.
    // TODO(Nimrod): Add global state tree.
    contract_states: HashMap<ContractAddress, S>,
}

#[allow(dead_code)]
pub(crate) fn create_original_skeleton_forest<O: OriginalSkeletonTree<LeafDataImpl>, S: Storage>(
    input: Input,
) -> BlockCommitmentResult<OriginalSkeletonForest<O>> {
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
            .ok_or_else(|| OriginalSkeletonTreeError::MissingInput(address))?;
        let cur_original_skeleton = O::create_tree(
            &actual_storage,
            &sorted_leaf_indices,
            cur_contract_state.storage_root_hash,
            input.tree_height,
        )?;
        contract_states.insert(address, cur_original_skeleton);
    }
    Ok(OriginalSkeletonForest { contract_states })
}
