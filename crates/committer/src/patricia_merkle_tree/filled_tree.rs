use std::collections::HashSet;

use crate::patricia_merkle_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::types::LeafDataTrait;
use crate::storage::storage_trait::Storage;

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// FilledTree consists of all nodes which were modified in the update, including their updated
/// data and hashes.
pub(crate) trait FilledTree<L: LeafDataTrait> {
    /// Serializes the tree into storage. Returns hash set of keys of the serialized nodes,
    /// if successful.
    fn serialize(&self, storage: &dyn Storage) -> Result<HashSet<&[u8]>, FilledTreeError>;
}
