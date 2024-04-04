use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::types::{LeafDataTrait, PathToBottom};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum SkeletonNode<L: LeafDataTrait + std::clone::Clone> {
    Binary,
    Edge { path_to_bottom: PathToBottom },
    Sibling(HashOutput),
    Leaf(L),
    Empty,
}
