use pathfinder_crypto::Felt;

use super::hash::{HashFunction, HashInput, HashOutput};

pub(crate) struct NodeIndex(pub Felt);

// TODO(Amos, 26/3/2024): Define trait's methods.
pub(crate) trait Node<H: HashFunction> {}

// TODO(Nimrod, 10/4/2024): Remove these statements once we deserailize from python.
#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub(crate) enum TreeNode {
    // Contains the storage data.
    LeafNode {
        value: Felt,
    },
    // Contains the hash of left and right sons at the tree.
    BinaryNode {
        left: Felt,
        right: Felt,
    },
    // Contains the (binary) path to the non-empty node it represents, the length of the path, and
    // it's value.
    EdgeNode {
        length: Felt,
        path: Felt,
        value: Felt,
    },
}

impl Node<DummyHashFunction> for TreeNode {}
// TODO(Nimrod, 1/4/2024): Remove these structs.
pub(crate) struct DummyHashFunction;
impl HashFunction for DummyHashFunction {
    fn compute_hash(i: HashInput) -> HashOutput {
        HashOutput(i.0 + i.1 + i.2)
    }
}
