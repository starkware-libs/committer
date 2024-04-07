use crate::patricia_merkle_tree::types::{LeafDataTrait, PathToBottom};
use crate::types::FeltTrait;
use crate::{hash::types::HashOutput, types::Felt};
// TODO(Nimrod, 1/6/2024): Swap to starknet-types-core types once implemented.
#[allow(dead_code)]
pub(crate) struct ClassHash(pub Felt);
#[allow(dead_code)]
pub(crate) struct Nonce(pub Felt);

#[allow(dead_code)]
pub(crate) struct FilledNode<L: LeafDataTrait> {
    hash: HashOutput,
    data: NodeData<L>,
}

#[allow(dead_code)]
pub(crate) enum NodeData<L: LeafDataTrait> {
    Binary(BinaryData),
    Edge(EdgeData),
    Leaf(L),
}

#[allow(dead_code)]
pub(crate) struct BinaryData {
    left_hash: HashOutput,
    right_hash: HashOutput,
}

#[allow(dead_code)]
pub(crate) struct EdgeData {
    bottom_hash: HashOutput,
    path_to_bottom: PathToBottom,
}

#[allow(dead_code)]
pub(crate) enum LeafData {
    StorageValue(Felt),
    CompiledClassHash(ClassHash),
    StateTreeTuple {
        class_hash: ClassHash,
        contract_state_root_hash: Felt,
        nonce: Nonce,
    },
}

impl LeafDataTrait for LeafData {
    fn is_empty(&self) -> bool {
        match self {
            LeafData::StorageValue(value) => *value == Felt::ZERO,
            LeafData::CompiledClassHash(class_hash) => class_hash.0 == Felt::ZERO,
            LeafData::StateTreeTuple {
                class_hash,
                contract_state_root_hash,
                nonce,
            } => {
                nonce.0 == Felt::ZERO
                    && class_hash.0 == Felt::ZERO
                    && *contract_state_root_hash == Felt::ZERO
            }
        }
    }
}

impl FilledNode<LeafData>{

    pub(crate) fn serialise(&self,) -> Vec<u8> {
        // This method serialises the filled node into a byte vector, where:
        // - For binary nodes: Concatenates left and right hashes.
        // - For edge nodes: Concatenates bottom hash, path, and path length.
        // - For leaf nodes: Concatenates the node's hash.

        let mut serialised = vec![];
        match &self.data {
            NodeData::Binary(data) => {
                serialised.extend_from_slice(&data.left_hash.0.to_bytes());
                serialised.extend_from_slice(&data.right_hash.0.to_bytes());
            }

            NodeData::Edge(data) => {
                serialised.extend_from_slice(&data.bottom_hash.0.to_bytes());
                serialised.extend_from_slice(&data.path_to_bottom.path.0.to_bytes());
                serialised.extend_from_slice(&data.path_to_bottom.length.0.to_be_bytes());
            }

            NodeData::Leaf(data) => {
                serialised.extend_from_slice(&self.hash.0.to_bytes());
                }
            }
        serialised
    }
}
