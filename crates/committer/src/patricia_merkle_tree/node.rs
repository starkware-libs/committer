use super::hash::{HashFunction, HashInput, HashOutput};
use pathfinder_crypto::Felt;
use serde::Deserialize;
#[derive(Debug)]
pub(crate) struct NodeIndex(pub Felt);
pub(crate) type IndexedNode = (NodeIndex, TreeNode);

// TODO(Amos, 26/3/2024): Define trait's methods.
pub(crate) trait Node<H: HashFunction> {}

// TODO(Nimrod, 10/4/2024): Remove these statements once we deserailize from python.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) enum TreeNode {
    // Contains the storage data.
    Leaf { value: Felt },
    // Contains the hash of left and right sons at the tree.
    Binary { left: Felt, right: Felt },
    // Contains the (binary) path to the non-empty node it represents, the length of the path, and
    // it's value.
    Edge { length: u8, path: Felt, value: Felt },
}

// impl TreeNodeTraits for TreeNode {}

impl Node<DummyHashFunction> for TreeNode {}
// TODO(Nimrod, 1/4/2024): Remove these structs.
pub(crate) struct DummyHashFunction;
impl HashFunction for DummyHashFunction {
    fn compute_hash(i: HashInput) -> HashOutput {
        HashOutput(i.0 + i.1 + i.2)
    }
}
