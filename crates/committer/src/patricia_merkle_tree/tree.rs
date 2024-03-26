use std::iter::Map;

use crate::patricia_merkle_tree::node::{Node, NodeIndex};

use crate::hash::types::{HashFunction, HashOutput};

use super::errors::TreeHashComputationError;

#[allow(dead_code)]
pub(crate) struct RootHash(HashOutput);

pub(crate) trait Tree<H: HashFunction, N: Node<H>> {
    /// Computes and sets hashes in entire tree and returns root hash, if possible.
    fn compute_and_set_all_hashes(&self) -> Result<RootHash, TreeHashComputationError> {
        if let Some(root) = self.get_root() {
            let root_hash = root.compute_and_set_hash_and_value_recursively()?;
            return Ok(RootHash(root_hash));
        }
        let empty_tree_root_hash = N::compute_hash(None)?;
        Ok(RootHash(empty_tree_root_hash))
    }

    /// Returns the root if the tree is not empty.
    fn get_root(&self) -> Option<&mut N>;

    /// Updates tree in place with given leaves, and returns all modified (and new) nodes.
    fn update_and_get_modified_nodes(
        &mut self,
        index_to_updated_leaf: Map<NodeIndex, &N>,
    ) -> Map<NodeIndex, &N>;
}
