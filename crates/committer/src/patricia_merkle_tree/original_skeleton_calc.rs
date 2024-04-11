use bisection::{bisect_left, bisect_right};

use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::errors::OriginalSkeletonTreeError;
use crate::patricia_merkle_tree::filled_node::BinaryData;
use crate::patricia_merkle_tree::filled_node::FilledNode;
use crate::patricia_merkle_tree::filled_node::NodeData;
use crate::patricia_merkle_tree::original_skeleton_tree::OriginalSkeletonTreeResult;
use crate::patricia_merkle_tree::types::EdgeData;
use crate::patricia_merkle_tree::types::TreeHeight;
use crate::patricia_merkle_tree::{
    filled_node::LeafData, original_skeleton_node::OriginalSkeletonNode, types::NodeIndex,
};
use crate::storage::storage_trait::Storage;
use crate::storage::storage_trait::StorageKey;
use crate::storage::storage_trait::StoragePrefix;
use crate::types::Felt;
use std::collections::HashMap;

#[cfg(test)]
#[path = "original_skeleton_calc_test.rs"]
pub mod original_skeleton_calc_test;

#[allow(dead_code)]
pub(crate) struct OriginalSkeletonTreeImpl {
    nodes: HashMap<NodeIndex, OriginalSkeletonNode<LeafData>>,
    leaf_modifications: HashMap<NodeIndex, LeafData>,
    tree_height: TreeHeight,
}

#[allow(dead_code)]
struct SubTree<'a> {
    pub sorted_leaf_indices: &'a [NodeIndex],
    pub root_index: NodeIndex,
    pub root_hash: HashOutput,
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
        let leftmost_index_in_right_subtree = ((self.root_index.times_two_to_the_power(1))
            + NodeIndex(Felt::ONE))
        .times_two_to_the_power(height.0 - 1);
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

#[allow(dead_code)]
impl OriginalSkeletonTreeImpl {
    /// Fetches the Patricia witnesses, required to build the original skeleton tree from storage.
    /// Given a list of subtrees, traverses towards their leaves and fetches all non-empty and
    /// sibling nodes. Assumes no subtrees of height 0 (leaves).

    fn fetch_nodes(
        &mut self,
        subtrees: Vec<SubTree<'_>>,
        storage: &impl Storage,
    ) -> OriginalSkeletonTreeResult<()> {
        if subtrees.is_empty() {
            return Ok(());
        }
        let mut next_subtrees = Vec::new();
        let mut subtrees_roots = Vec::new();
        for subtree in subtrees.iter() {
            let key =
                StorageKey::from(subtree.root_hash.0).with_prefix(StoragePrefix::PatriciaNode);
            let val = storage
                .get(&key)
                .ok_or(OriginalSkeletonTreeError::StorageRead(key))?;
            subtrees_roots.push(FilledNode::deserialize(
                &StorageKey::from(subtree.root_hash.0),
                val,
            )?)
        }
        for (filled_node, subtree) in subtrees_roots.into_iter().zip(subtrees.iter()) {
            match filled_node.data {
                // Binary node.
                NodeData::Binary(BinaryData {
                    left_hash,
                    right_hash,
                }) => {
                    if subtree.is_sibling() {
                        self.nodes.insert(
                            subtree.root_index,
                            OriginalSkeletonNode::LeafOrBinarySibling(filled_node.hash),
                        );
                        continue;
                    }
                    self.nodes
                        .insert(subtree.root_index, OriginalSkeletonNode::Binary);
                    let (left_leaves, right_leaves) = subtree.split_leaves(&self.tree_height);
                    let left_root_index = subtree.root_index.times_two_to_the_power(1);
                    let left_subtree = SubTree {
                        sorted_leaf_indices: left_leaves,
                        root_index: left_root_index,
                        root_hash: left_hash,
                    };
                    let right_subtree = SubTree {
                        sorted_leaf_indices: right_leaves,
                        root_index: left_root_index + NodeIndex(Felt::ONE),
                        root_hash: right_hash,
                    };
                    if subtree.get_height(&self.tree_height).is_of_height_one() {
                        // Children are leaves.
                        if left_subtree.is_sibling() {
                            self.nodes.insert(
                                left_subtree.root_index,
                                OriginalSkeletonNode::LeafOrBinarySibling(left_hash),
                            );
                        }
                        if right_subtree.is_sibling() {
                            self.nodes.insert(
                                right_subtree.root_index,
                                OriginalSkeletonNode::LeafOrBinarySibling(right_hash),
                            );
                        }
                        continue;
                    }
                    next_subtrees.extend(vec![left_subtree, right_subtree]);
                }
                // Edge node.
                NodeData::Edge(EdgeData {
                    bottom_hash,
                    path_to_bottom,
                }) => {
                    if subtree.is_sibling() {
                        // Sibling will remain an edge node. No need to open the bottom.
                        self.nodes.insert(
                            subtree.root_index,
                            OriginalSkeletonNode::EdgeSibling(EdgeData {
                                bottom_hash,
                                path_to_bottom,
                            }),
                        );
                        continue;
                    }
                    // Parse bottom.
                    let bottom_index = path_to_bottom.bottom_index(subtree.root_index);
                    let bottom_height =
                        subtree.get_height(&self.tree_height) - TreeHeight(path_to_bottom.length.0);
                    let leftmost_in_subtree = bottom_index.times_two_to_the_power(bottom_height.0);
                    let rightmost_in_subtree = leftmost_in_subtree
                        + (NodeIndex(Felt::ONE).times_two_to_the_power(bottom_height.0))
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
                                OriginalSkeletonNode::LeafOrBinarySibling(bottom_hash),
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
                NodeData::Leaf(_) => {
                    return Err(OriginalSkeletonTreeError::LeafEncountered(
                        subtree.root_index,
                    ))
                }
            }
        }
        self.fetch_nodes(next_subtrees, storage)
    }
}
