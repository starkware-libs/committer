use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::types::{Leaf, PathToBottom};

#[allow(dead_code)]
pub(crate) enum SkeletonNode<L: Leaf> {
    Binary,
    Edge { path_to_bottom: PathToBottom },
    Sibling(HashOutput),
    Leaf(L),
}
