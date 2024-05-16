use std::collections::HashMap;
use std::marker::PhantomData;

use crate::block_committer::input::ContractAddress;
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTree;

#[allow(dead_code)]
pub(crate) struct UpdatedSkeletonForest<
    L: LeafData + std::clone::Clone + Send + Sync,
    O: OriginalSkeletonTree<L>,
    U: UpdatedSkeletonTree<L, O>,
> {
    #[allow(dead_code)]
    classes_tree: U,
    #[allow(dead_code)]
    global_state_tree: U,
    #[allow(dead_code)]
    contract_states: HashMap<ContractAddress, U>,
    leaf_data: PhantomData<L>,
    original_tree_data: PhantomData<O>,
}
