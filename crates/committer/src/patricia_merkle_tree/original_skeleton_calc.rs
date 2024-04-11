use bisection::{bisect_left, bisect_right};

use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::original_skeleton_tree::OriginalSkeletonTreeResult;
use crate::patricia_merkle_tree::types::EdgeData;
use crate::patricia_merkle_tree::types::TreeHeight;
use crate::patricia_merkle_tree::{
    filled_node::LeafData, original_skeleton_node::OriginalSkeletonNode, types::NodeIndex,
};
use crate::storage::storage_trait::{StorageKey, StorageValue};
use crate::types::Felt;
use std::collections::HashMap;

use super::errors::OriginalSkeletonTreeError;

#[allow(dead_code)]
pub(crate) struct SkeletonTree<'a> {
    nodes: HashMap<NodeIndex, OriginalSkeletonNode<LeafData>>,
    leaf_modifications: &'a HashMap<NodeIndex, LeafData>,
    tree_height: TreeHeight,
}
#[allow(dead_code)]
impl SkeletonTree<'_> {
    fn fetch_nodes(
        &mut self,
        subtrees: Vec<SubTree<'_>>,
        storage: HashMap<StorageKey, StorageValue>,
        // TODO(Nimrod, 25/4/2024): Change input type to Storage once Amos has implemented the
        // Storage trait for HashMap.
    ) -> OriginalSkeletonTreeResult<()> {
        if subtrees.is_empty() {
            return Ok(());
        }
        let mut next_subtrees = Vec::new();
        let root_vals: Vec<&StorageValue> = (subtrees
            .iter()
            .map(|subtree| {
                storage
                    .get(&subtree.root_hash.with_patricia_prefix())
                    .ok_or(OriginalSkeletonTreeError::StorageRead)
            })
            .collect::<OriginalSkeletonTreeResult<Vec<_>>>())?;
        for (root_val, subtree) in root_vals.iter().zip(subtrees.iter()) {
            if root_val.is_binary_node() {
                // Binary node.
                if subtree.is_sibling() {
                    self.nodes.insert(
                        subtree.root_index,
                        OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(
                            Felt::from_bytes_be_slice(&subtree.root_hash.0),
                        )),
                    );
                    continue;
                }
                self.nodes
                    .insert(subtree.root_index, OriginalSkeletonNode::Binary);
                let (left_key, right_key) = root_val.deserialize_binary_node()?;
                let (left_leaves, right_leaves) = subtree.split_leaves(&self.tree_height);
                let left_root_index = subtree.root_index.shift_left(1);
                let left_subtree = SubTree {
                    sorted_leaf_indices: left_leaves,
                    root_index: left_root_index,
                    root_hash: left_key,
                };
                let right_subtree = SubTree {
                    sorted_leaf_indices: right_leaves,
                    root_index: left_root_index + NodeIndex(Felt::ONE),
                    root_hash: right_key,
                };
                if subtree.get_height(&self.tree_height).is_of_height_one() {
                    // Children are leaves.
                    if left_subtree.is_sibling() {
                        self.nodes.insert(
                            left_subtree.root_index,
                            OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(
                                Felt::from_bytes_be_slice(&left_subtree.root_hash.0),
                            )),
                        );
                    }
                    if right_subtree.is_sibling() {
                        self.nodes.insert(
                            right_subtree.root_index,
                            OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(
                                Felt::from_bytes_be_slice(&right_subtree.root_hash.0),
                            )),
                        );
                    }
                    continue;
                }
                next_subtrees.extend(vec![left_subtree, right_subtree]);
            } else {
                let (bottom_hash, path_to_bottom) = root_val.deserialize_edge_node()?;
                if subtree.is_sibling() {
                    // Sibling will remain an edge node. No need to open the bottom.
                    self.nodes.insert(
                        subtree.root_index,
                        OriginalSkeletonNode::EdgeSibling(EdgeData {
                            bottom_hash: HashOutput(Felt::from_bytes_be_slice(&bottom_hash.0)),
                            path_to_bottom,
                        }),
                    );
                    continue;
                }
                // Parse bottom.
                let bottom_index = path_to_bottom.bottom_index(subtree.root_index);
                let bottom_height =
                    subtree.get_height(&self.tree_height) - TreeHeight(path_to_bottom.length.0);
                let leftmost_in_subtree = bottom_index.shift_left(bottom_height.0);
                let rightmost_in_subtree = leftmost_in_subtree
                    + (NodeIndex(Felt::ONE).shift_left(bottom_height.0))
                    - NodeIndex(Felt::ONE);
                let bottom_leaves = &subtree.sorted_leaf_indices[bisect_left(
                    subtree.sorted_leaf_indices,
                    &leftmost_in_subtree,
                )
                    ..bisect_right(subtree.sorted_leaf_indices, &rightmost_in_subtree)];
                self.nodes.insert(
                    subtree.root_index,
                    OriginalSkeletonNode::Edge { path_to_bottom },
                );
                if bottom_height.is_leaf_height() {
                    if bottom_leaves.is_empty() {
                        // Bottom is a leaf sibling.
                        self.nodes.insert(
                            bottom_index,
                            OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(
                                Felt::from_bytes_be_slice(&bottom_hash.0),
                            )),
                        );
                    }
                    continue;
                }
                let bottom_subtree = SubTree {
                    sorted_leaf_indices: bottom_leaves,
                    root_index: bottom_index,
                    root_hash: bottom_hash,
                };
                next_subtrees.push(bottom_subtree);
            }
        }
        self.fetch_nodes(next_subtrees, storage)
    }
}
#[allow(dead_code)]
struct SubTree<'a> {
    pub sorted_leaf_indices: &'a [NodeIndex],
    pub root_index: NodeIndex,
    pub root_hash: StorageKey,
}
#[allow(dead_code)]
impl<'a> SubTree<'a> {
    pub(crate) fn get_height(&self, total_tree_height: &TreeHeight) -> TreeHeight {
        TreeHeight(total_tree_height.0 - self.root_index.0.bits() + 1)
    }

    pub(crate) fn split_leaves(
        &self,
        total_tree_height: &TreeHeight,
    ) -> (&'a [NodeIndex], &'a [NodeIndex]) {
        let height = self.get_height(total_tree_height);
        let leftmost_index_in_right_subtree =
            ((self.root_index.shift_left(1)) + NodeIndex(Felt::ONE)).shift_left(height.0 - 1);
        let mid = bisect_left(self.sorted_leaf_indices, &leftmost_index_in_right_subtree);
        (
            &self.sorted_leaf_indices[..mid],
            &self.sorted_leaf_indices[mid..],
        )
    }

    pub(crate) fn is_sibling(&self) -> bool {
        self.sorted_leaf_indices.is_empty()
    }
}

#[cfg(test)]
#[path = "original_skeleton_calc_test.rs"]
pub mod original_skeleton_calc_test;
