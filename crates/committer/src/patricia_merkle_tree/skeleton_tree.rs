use std::iter::Map;

use crate::patricia_merkle_tree::errors::SkeletonTreeError;
use crate::patricia_merkle_tree::filled_tree::FilledTree;
use crate::patricia_merkle_tree::skeleton_node::SkeletonNode;
use crate::patricia_merkle_tree::types::{Leaf, NodeIndex, TreeHashFunction};

pub(crate) trait CurrentSkeletonTree<L: Leaf, H: TreeHashFunction<L>> {
    /// Computes and returns updated skeleton tree.
    fn compute_updated_skeleton_tree(
        &self,
        index_to_updated_leaf: Map<NodeIndex, &SkeletonNode<L>>,
    ) -> Result<SkeletonTreeError, impl UpdatedSkeletonTree<L, H>>;
}

pub(crate) trait UpdatedSkeletonTree<L: Leaf, H: TreeHashFunction<L>> {
    /// Computes and returns the filled tree.
    fn compute_filled_tree(&self) -> Result<SkeletonTreeError, impl FilledTree<L>>;
}
