use std::collections::HashMap;

use crate::patricia_merkle_tree::node_data::leaf::{LeafModifications, SkeletonLeaf};
use crate::patricia_merkle_tree::original_skeleton_tree::node::OriginalSkeletonNode;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::types::{NodeIndex, TreeHeight};
use crate::patricia_merkle_tree::updated_skeleton_tree::compute_updated_skeleton_tree::TempSkeletonNode;
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
        original_skeleton: &mut impl OriginalSkeletonTree,
        leaf_modifications: &LeafModifications<SkeletonLeaf>,
    ) -> UpdatedSkeletonTreeResult<Self>;

    /// Does the skeleton represents an empty-tree (i.e. all leaves are empty).
    #[allow(dead_code)]
    fn is_empty(&self) -> bool;

    /// Returns an iterator over all (node index, node) pairs in the tree.
    #[allow(dead_code)]
    fn get_nodes(&self) -> impl Iterator<Item = (NodeIndex, UpdatedSkeletonNode)>;

    /// Returns the node with the given index.
    #[allow(dead_code)]
    fn get_node(&self, index: NodeIndex) -> UpdatedSkeletonTreeResult<&UpdatedSkeletonNode>;
}

pub(crate) struct UpdatedSkeletonTreeImpl {
    pub(crate) tree_height: TreeHeight,
    pub(crate) skeleton_tree: HashMap<NodeIndex, UpdatedSkeletonNode>,
}

impl UpdatedSkeletonTree for UpdatedSkeletonTreeImpl {
    fn create(
        original_skeleton: &mut impl OriginalSkeletonTree,
        leaf_modifications: &LeafModifications<SkeletonLeaf>,
    ) -> UpdatedSkeletonTreeResult<Self> {
        // Finalize modified leaves, and unmodified nodes (Siblings and UnmodifiedBottoms) in the
        // skeleton.
        let skeleton_tree: HashMap<NodeIndex, UpdatedSkeletonNode> =
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
        let mut updated_skeleton_tree = UpdatedSkeletonTreeImpl {
            tree_height: *original_skeleton.get_tree_height(),
            skeleton_tree,
        };

        // Finalize the tree middle layers (i.e., not the root or
        // leaves/siblings/unmodified_bottoms).
        let leaf_indices: Vec<NodeIndex> = leaf_modifications.keys().cloned().collect();
        let temp_root = if original_skeleton.get_nodes().is_empty() {
            updated_skeleton_tree.update_node_in_empty_tree(&NodeIndex::ROOT, &leaf_indices)
        } else {
            updated_skeleton_tree.update_node_in_nonempty_tree(
                &NodeIndex::ROOT,
                original_skeleton.get_nodes_mut(),
                &leaf_indices,
            )
        };

        // Finalize root.
        match temp_root {
            TempSkeletonNode::Empty => {}
            TempSkeletonNode::Leaf => {
                unreachable!("Root node cannot be a leaf")
            }
            TempSkeletonNode::Original(original_skeleton_node) => {
                updated_skeleton_tree
                    .skeleton_tree
                    .insert(
                        NodeIndex::ROOT,
                        match original_skeleton_node {
                            OriginalSkeletonNode::Binary => UpdatedSkeletonNode::Binary,
                            OriginalSkeletonNode::Edge(path_to_bottom) => {
                                UpdatedSkeletonNode::Edge(path_to_bottom)
                            }
                            OriginalSkeletonNode::LeafOrBinarySibling(_) => {
                                unreachable!("Root node cannot be a sibling")
                            }
                            OriginalSkeletonNode::UnmodifiedBottom(_) => {
                                unreachable!("Root node cannot be an unmodified bottom")
                            }
                        },
                    )
                    .or_else(|| panic!("Root node already exists in the updated skeleton tree"));
            }
        };
        Ok(updated_skeleton_tree)
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
