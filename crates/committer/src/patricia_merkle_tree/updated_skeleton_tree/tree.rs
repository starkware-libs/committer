use std::collections::HashMap;

use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::updated_skeleton_tree::errors::UpdatedSkeletonTreeError;
use crate::patricia_merkle_tree::updated_skeleton_tree::node::UpdatedSkeletonNode;

#[cfg(test)]
#[path = "tree_test.rs"]
pub mod tree_test;

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// This trait represents the structure of the subtree which was modified in the update.
/// It also contains the hashes of the Sibling nodes on the Merkle paths from the updated leaves
/// to the root.
pub(crate) trait UpdatedSkeletonTree<
    L: LeafData + std::clone::Clone + Sync + Send,
    T: OriginalSkeletonTree<L>,
>: Sized
{
    #[allow(dead_code)]
    /// Computes and returns updated skeleton tree.
    fn create(
        original_skeleton_tree: T,
        index_to_updated_leaf: HashMap<NodeIndex, L>,
    ) -> Result<Self, UpdatedSkeletonTreeError<L>>;

    fn get_node(
        &self,
        index: NodeIndex,
    ) -> Result<&UpdatedSkeletonNode<L>, UpdatedSkeletonTreeError<L>>;

    fn get_nodes<'a>(&'a self) -> impl Iterator<Item = (&'a NodeIndex, &'a UpdatedSkeletonNode<L>)>
    where
        L: 'a;
}

pub(crate) struct UpdatedSkeletonTreeImpl<L: LeafData + std::clone::Clone> {
    skeleton_tree: HashMap<NodeIndex, UpdatedSkeletonNode<L>>,
}

impl<
        L: LeafData + std::clone::Clone + std::marker::Sync + std::marker::Send,
        T: OriginalSkeletonTree<L>,
    > UpdatedSkeletonTree<L, T> for UpdatedSkeletonTreeImpl<L>
{
    fn create(
        _original_skeleton_tree: T,
        _index_to_updated_leaf: HashMap<NodeIndex, L>,
    ) -> Result<Self, UpdatedSkeletonTreeError<L>> {
        todo!();
    }

    fn get_node(
        &self,
        index: NodeIndex,
    ) -> Result<&UpdatedSkeletonNode<L>, UpdatedSkeletonTreeError<L>> {
        match self.skeleton_tree.get(&index) {
            Some(node) => Ok(node),
            None => Err(UpdatedSkeletonTreeError::MissingNode(index)),
        }
    }

    fn get_nodes<'a>(&'a self) -> impl Iterator<Item = (&'a NodeIndex, &'a UpdatedSkeletonNode<L>)>
    where
        L: 'a,
    {
        self.skeleton_tree.iter()
    }
}
