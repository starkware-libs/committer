use std::collections::HashMap;

use crate::patricia_merkle_tree::node_data::leaf::{LeafModifications, SkeletonLeaf};
use crate::patricia_merkle_tree::original_skeleton_tree::node::OriginalSkeletonNode;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::types::{NodeIndex, TreeHeight};
use crate::patricia_merkle_tree::updated_skeleton_tree::errors::UpdatedSkeletonTreeError;
use crate::patricia_merkle_tree::updated_skeleton_tree::node::UpdatedSkeletonNode;

#[cfg(test)]
#[path = "tree_test.rs"]
pub mod tree_test;

pub(crate) type UpdatedSkeletonTreeResult<T> = Result<T, UpdatedSkeletonTreeError>;

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// This trait represents the structure of the subtree which was modified in the update.
/// It also contains the hashes of the Sibling nodes on the Merkle paths from the updated leaves
/// to the root.
pub(crate) trait UpdatedSkeletonTree: Sized + Send + Sync {
    /// Creates an updated tree from an original tree and modifications.
    fn create(
        original_skeleton: &impl OriginalSkeletonTree,
        leaf_modifications: &LeafModifications<SkeletonLeaf>,
    ) -> UpdatedSkeletonTreeResult<Self>;

    /// Does the skeleton represents an empty-tree (i.e. all leaves are empty).
    fn is_empty(&self) -> bool;

    /// Returns an iterator over all (node index, node) pairs in the tree.
    fn get_nodes(&self) -> impl Iterator<Item = (NodeIndex, UpdatedSkeletonNode)>;

    /// Returns the node with the given index.
    fn get_node(&self, index: NodeIndex) -> UpdatedSkeletonTreeResult<&UpdatedSkeletonNode>;
}

pub(crate) struct UpdatedSkeletonTreeImpl {
    pub(crate) tree_height: TreeHeight,
    pub(crate) skeleton_tree: HashMap<NodeIndex, UpdatedSkeletonNode>,
}

impl UpdatedSkeletonTree for UpdatedSkeletonTreeImpl {
    fn create(
        original_skeleton: &impl OriginalSkeletonTree,
        leaf_modifications: &LeafModifications<SkeletonLeaf>,
    ) -> UpdatedSkeletonTreeResult<Self> {
        // Finalize modified leaves, and unmodified nodes (Siblings and UnmodifiedBottoms) in the
        // skeleton.
        let mut _skeleton_tree: HashMap<NodeIndex, UpdatedSkeletonNode> =
            leaf_modifications
                .iter()
                .filter(|(_, leaf)| !leaf.is_zero())
                .map(|(index, _)| (*index, UpdatedSkeletonNode::Leaf))
                .chain(original_skeleton.get_nodes().iter().filter_map(
                    |(index, node)| match node {
                        OriginalSkeletonNode::LeafOrBinarySibling(hash) => {
                            Some((*index, UpdatedSkeletonNode::Sibling(*hash)))
                        }
                        OriginalSkeletonNode::UnmodifiedBottom(hash) => {
                            Some((*index, UpdatedSkeletonNode::UnmodifiedBottom(*hash)))
                        }
                        OriginalSkeletonNode::Binary | OriginalSkeletonNode::Edge(_) => None,
                    },
                ))
                .collect();
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn get_node(&self, index: NodeIndex) -> UpdatedSkeletonTreeResult<&UpdatedSkeletonNode> {
        match self.skeleton_tree.get(&index) {
            Some(node) => Ok(node),
            None => Err(UpdatedSkeletonTreeError::MissingNode(index)),
        }
    }

    fn get_nodes(&self) -> impl Iterator<Item = (NodeIndex, UpdatedSkeletonNode)> {
        self.skeleton_tree
            .iter()
            .map(|(index, node)| (*index, node.clone()))
    }
}
