use std::iter::Map;

use crate::patricia_merkle_tree::node::{Node, NodeIndex};

use crate::hash::types::HashFunction;

pub(crate) trait Tree<H: HashFunction, LeafVal, N: Node<H, LeafVal>> {
    /// Returns the root if the tree is not empty.
    fn get_root(&self) -> Option<&mut N>;

    /// Updates tree in place with given leaves, and returns all modified (and new) nodes.
    fn update_and_get_modified_nodes(
        &mut self,
        index_to_updated_leaf: Map<NodeIndex, &N>,
    ) -> Map<NodeIndex, &N>;
}
