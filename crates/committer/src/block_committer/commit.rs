use std::collections::HashMap;

use crate::block_committer::errors::BlockCommitmentError;
use crate::block_committer::types::Input;
use crate::patricia_merkle_tree::original_skeleton_tree::original_skeleton_calc::OriginalSkeletonTreeImpl;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::map_storage::MapStorage;

type BlockCommitmentResult<T> = Result<T, BlockCommitmentError>;

#[allow(dead_code)]
pub(crate) fn commit_block(input: Input) -> BlockCommitmentResult<Vec<OriginalSkeletonTreeImpl>> {
    let actual_storage = MapStorage::from(input.storage);
    let mut original_skeletons = vec![];
    let accessed_addresses = input.state_diff.accessed_addresses();
    for address in accessed_addresses {
        let mut sorted_leaf_indices: Vec<NodeIndex> = input
            .state_diff
            .storage_updates
            .get(address)
            .unwrap_or(&HashMap::new())
            .keys()
            .map(|key| NodeIndex::from(*key).as_full_index(&input.tree_height))
            .collect();
        sorted_leaf_indices.sort();
        let cur_contract_state = input
            .state_diff
            .current_contract_state_leaves
            .get(address)
            .ok_or_else(|| BlockCommitmentError::LowerTreeCommitmentError(*address))?;
        let cur_original_skeleton = OriginalSkeletonTreeImpl::create_tree(
            &actual_storage,
            &sorted_leaf_indices,
            cur_contract_state.storage_root_hash,
            input.tree_height,
        )?;
        original_skeletons.push(cur_original_skeleton);
    }
    Ok(original_skeletons)
}
