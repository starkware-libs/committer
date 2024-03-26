use std::iter::Map;

use crate::patricia_merkle_tree::node::{Node, NodeIndex};

use crate::hash::types::{HashFunction, HashOutput};

use super::errors::TreeHashComputationError;

pub(crate) trait Tree<H: HashFunction, N: Node<H>> {
    /// Computes and sets hashes in entire tree and returns root hash, if possible.
    fn compute_and_set_all_hashes(&self) -> Result<HashOutput, TreeHashComputationError> {
        if let Some(root) = self.get_root() {
            return root.compute_and_set_hash_and_value_recursively();
        }
        N::compute_hash(None)
    }

    /// Returns the root if the tree is not empty.
    fn get_root(&self) -> Option<&mut N>;

    /// Updates tree in place with given leaves, and returns all modified (and new) nodes.
    fn update_and_get_modified_nodes(
        &mut self,
        index_to_updated_leaf: Map<NodeIndex, &N>,
    ) -> Map<NodeIndex, &N>;
}
