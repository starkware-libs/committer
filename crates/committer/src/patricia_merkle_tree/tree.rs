use std::iter::Map;

use crate::patricia_merkle_tree::node::{Node, NodeIndex};

use crate::hash::types::HashFunction;

pub(crate) trait Tree<H: HashFunction, N: Node<H>> {
    /// Returns the node with given full (Merkle) index, if it exists.
    fn get_node(&self, full_index: NodeIndex) -> Option<&N>;

    /// Returns the root if the tree is not empty.
    fn get_root(&mut self) -> Option<&mut N>;

    /// Updates tree in place with given leaves.
    fn update(&mut self, index_to_updated_leaf: Map<NodeIndex, &N>);

    /// Returns all nodes added or modified in the last update.
    fn get_all_new_or_modified_nodes(&self) -> Map<NodeIndex, &N>;
}
