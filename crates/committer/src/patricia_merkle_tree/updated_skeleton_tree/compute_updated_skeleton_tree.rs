use crate::patricia_merkle_tree::node_data::inner_node::EdgeData;
use crate::patricia_merkle_tree::node_data::inner_node::PathToBottom;
use crate::patricia_merkle_tree::node_data::leaf::LeafModifications;
use crate::patricia_merkle_tree::node_data::leaf::SkeletonLeaf;
use crate::patricia_merkle_tree::original_skeleton_tree::node::OriginalSkeletonNode;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTreeImpl;
use crate::patricia_merkle_tree::original_skeleton_tree::utils::split_leaves;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::updated_skeleton_tree::node::UpdatedSkeletonNode;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTreeImpl;

#[cfg(test)]
#[path = "compute_updated_skeleton_tree_test.rs"]
pub mod compute_updated_skeleton_tree_test;

#[derive(Debug, PartialEq, Eq)]
/// A temporary skeleton node used during the computation of the updated skeleton tree.
enum TempSkeletonNode {
    Empty,
    #[allow(dead_code)]
    Original(OriginalSkeletonNode),
}

impl TempSkeletonNode {
    fn is_empty(&self) -> bool {
        *self == TempSkeletonNode::Empty
    }
}

impl OriginalSkeletonTreeImpl {
    #[allow(dead_code)]
    /// Returns the path from the given root_index to the LCA of the leaves. Assumes the leaves are:
    /// * Sorted.
    /// * Descendants of the given index.
    /// * Non-empty list.
    fn get_path_to_lca(&self, root_index: &NodeIndex, leaf_indices: &[NodeIndex]) -> PathToBottom {
        if leaf_indices.is_empty() {
            panic!("Unexpected empty array.");
        }
        let lca = if leaf_indices.len() == 1 {
            leaf_indices[0]
        } else {
            leaf_indices[0].get_lca(leaf_indices.last().expect("Unexpected empty array"))
        };
        root_index.get_path_to_descendant(lca)
    }

    /// Returns whether a root of a subtree has leaves on both sides. Assumes:
    /// * The leaf indices array is sorted.
    /// * All leaves are descendants of the root.
    #[allow(dead_code)]
    fn has_leaves_on_both_sides(&self, root_index: &NodeIndex, leaf_indices: &[NodeIndex]) -> bool {
        if leaf_indices.is_empty() {
            return false;
        }
        split_leaves(&self.tree_height, root_index, leaf_indices)
            .iter()
            .all(|leaves_in_side| !leaves_in_side.is_empty())
    }
}

impl UpdatedSkeletonTreeImpl {
    #[allow(dead_code)]
    /// Builds a (probably binary) node from its two updated children. Returns the TempSkeletonNode
    /// matching the given root for the subtree it is the root of. If one or more children are
    /// empty, the resulting node will not be binary.
    fn node_from_binary_data(
        &mut self,
        root_index: &NodeIndex,
        left: &TempSkeletonNode,
        right: &TempSkeletonNode,
        leaf_modifications: &LeafModifications<SkeletonLeaf>,
    ) -> TempSkeletonNode {
        let [left_index, right_index] = root_index.get_children_indices();

        if !left.is_empty() && !right.is_empty() {
            // Both children are non-empty - a binary node.
            // Finalize children, as a binary node cannot change form.
            for (index, node) in [(left_index, left), (right_index, right)] {
                let updated = match node {
                    TempSkeletonNode::Empty => unreachable!("Unexpected empty node."),
                    // Leaf is finalized upon updated skeleton creation.
                    TempSkeletonNode::Original(OriginalSkeletonNode::Leaf(_)) => continue,
                    TempSkeletonNode::Original(OriginalSkeletonNode::Binary) => {
                        UpdatedSkeletonNode::Binary
                    }
                    TempSkeletonNode::Original(OriginalSkeletonNode::Edge { path_to_bottom }) => {
                        UpdatedSkeletonNode::Edge {
                            path_to_bottom: *path_to_bottom,
                        }
                    }
                    TempSkeletonNode::Original(OriginalSkeletonNode::LeafOrBinarySibling(hash)) => {
                        UpdatedSkeletonNode::Sibling(*hash)
                    }
                    TempSkeletonNode::Original(OriginalSkeletonNode::EdgeSibling(EdgeData {
                        path_to_bottom,
                        bottom_hash,
                    })) => {
                        self.skeleton_tree.insert(
                            path_to_bottom.bottom_index(index),
                            UpdatedSkeletonNode::Sibling(*bottom_hash),
                        );
                        UpdatedSkeletonNode::Edge {
                            path_to_bottom: *path_to_bottom,
                        }
                    }
                };
                self.skeleton_tree.insert(index, updated);
            }

            return TempSkeletonNode::Original(OriginalSkeletonNode::Binary);
        }

        // At least one of the children is empty.
        let (child_node, child_index, child_direction) = if *right == TempSkeletonNode::Empty {
            (left, left_index, PathToBottom::LEFT_CHILD)
        } else {
            (right, right_index, PathToBottom::RIGHT_CHILD)
        };
        self.node_from_edge_data(
            &child_direction,
            &child_index,
            child_node,
            leaf_modifications,
        )
    }

    /// Builds a (probably edge) node from its given updated descendant. Returns the
    /// TempSkeletonNode matching the given root (the source for the path to bottom) for the subtree
    /// it is the root of. If bottom is empty, returns an empty node.
    fn node_from_edge_data(
        &mut self,
        path: &PathToBottom,
        bottom_index: &NodeIndex,
        bottom: &TempSkeletonNode,
        leaf_modifications: &LeafModifications<SkeletonLeaf>,
    ) -> TempSkeletonNode {
        TempSkeletonNode::Original(match bottom {
            TempSkeletonNode::Empty => return TempSkeletonNode::Empty,
            TempSkeletonNode::Original(OriginalSkeletonNode::Leaf(_)) => {
                let leaf = leaf_modifications
                    .get(bottom_index)
                    .unwrap_or_else(|| panic!("Leaf modification {bottom_index:?} not found"));
                if leaf.is_zero() {
                    return TempSkeletonNode::Empty;
                };
                OriginalSkeletonNode::Edge {
                    path_to_bottom: *path,
                }
            }
            TempSkeletonNode::Original(OriginalSkeletonNode::Edge { path_to_bottom }) => {
                OriginalSkeletonNode::Edge {
                    path_to_bottom: path.concat_paths(*path_to_bottom),
                }
            }
            TempSkeletonNode::Original(OriginalSkeletonNode::EdgeSibling(edge_data)) => {
                OriginalSkeletonNode::EdgeSibling(EdgeData {
                    bottom_hash: edge_data.bottom_hash,
                    path_to_bottom: path.concat_paths(edge_data.path_to_bottom),
                })
            }
            TempSkeletonNode::Original(OriginalSkeletonNode::Binary) => {
                // Finalize bottom - a binary descendant cannot change form.
                self.skeleton_tree
                    .insert(*bottom_index, UpdatedSkeletonNode::Binary);
                OriginalSkeletonNode::Edge {
                    path_to_bottom: *path,
                }
            }
            TempSkeletonNode::Original(OriginalSkeletonNode::LeafOrBinarySibling(_)) => {
                OriginalSkeletonNode::Edge {
                    path_to_bottom: *path,
                }
            }
        })
    }
}
