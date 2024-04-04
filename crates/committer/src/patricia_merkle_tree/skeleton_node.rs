use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::types::{LeafDataTrait, PathToBottom};

#[allow(dead_code)]
/// A node in the structure of a Patricia-Merkle tree, before or after an update.
#[derive(Debug, Clone)]
pub(crate) enum SkeletonNode<L: LeafDataTrait + std::clone::Clone> {
    Binary,
    Edge { path_to_bottom: PathToBottom },
    // This type includes all unmodified nodes on the merkle paths of modified leaves.
    Sibling(HashOutput),
    Leaf(L),
    Empty,
}
