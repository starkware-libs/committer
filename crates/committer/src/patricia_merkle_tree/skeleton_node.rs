use starknet_types_core::felt::Felt;

use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::types::{LeafDataTrait, NodeIndex, PathToBottom};

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

pub(crate) fn compute_bottom_index(index: NodeIndex, path_to_bottom: PathToBottom) -> NodeIndex {
    let mut bottom_index = index.0;
    let PathToBottom { path, length } = path_to_bottom;
    for _ in 0..length.0 {
        bottom_index *= Felt::TWO;
    }
    bottom_index += path.0;
    NodeIndex(bottom_index)
}
