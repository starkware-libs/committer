use starknet_types_core::felt::FromStrError;

use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::node_data::inner_node::NodeData;
use crate::patricia_merkle_tree::node_data::leaf::LeafData;

// TODO(Nimrod, 1/6/2024): Swap to starknet-types-core types once implemented.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClassHash(pub Felt);

impl ClassHash {
    pub(crate) fn from_hex(hex_string: &str) -> Result<Self, FromStrError> {
        Ok(Self(Felt::from_hex(hex_string)?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Nonce(pub Felt);

impl Nonce {
    pub(crate) fn from_hex(hex_string: &str) -> Result<Self, FromStrError> {
        Ok(Self(Felt::from_hex(hex_string)?))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompiledClassHash(pub Felt);

#[derive(Clone, Debug, PartialEq, Eq)]
/// A node in a Patricia-Merkle tree which was modified during an update.
pub struct FilledNode<L: LeafData> {
    pub hash: HashOutput,
    pub data: NodeData<L>,
}
