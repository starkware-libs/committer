use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::types::{LeafDataTrait, PathToBottom};

#[allow(dead_code)]
/// A node in the structure of a Patricia-Merkle tree, after the update.
pub(crate) enum UpdatedSkeletonNode<L: LeafDataTrait> {
    Binary,
    // All unmodified nodes on the merkle paths of modified leaves.
    Edge { path_to_bottom: PathToBottom },
    Sibling(HashOutput),
    Leaf(L),
}
