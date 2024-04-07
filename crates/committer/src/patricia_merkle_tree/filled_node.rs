use crate::patricia_merkle_tree::types::{LeafDataTrait, PathToBottom};
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

    let mut fact = vec![];
    match &self.data {
        NodeData::Binary(data) => {
            fact.extend_from_slice(&data.left_hash);
            fact.extend_from_slice(&data.right_hash);
        }
        NodeData::Edge(data) => {
            fact.extend_from_slice(&data.bottom_hash);
            fact.extend_from_slice(&data.path_to_bottom.path.0.to_be_bytes());
            fact.extend_from_slice(&data.path_to_bottom.length.to_be_bytes());
        }
        NodeData::Leaf(data) => {
            match data {
                //``
                LeafData::StorageValue(value) => {
                    fact.extend_from_slice(&value.to_be_bytes());
                }
                LeafData::CompiledClassHash(class_hash) => {
                    fact.extend_from_slice(&class_hash.0.to_be_bytes());
                }
                LeafData::StateTreeTuple {
                    class_hash,
                    contract_state_root_hash,
                    nonce,
                } => {
                    //TODO: Aviv(4.4.2024) - Change StateTreeTuple implementation to be as python.
                    fact.extend_from_slice(&class_hash.0.to_be_bytes());
                    fact.extend_from_slice(&contract_state_root_hash.to_be_bytes());
                }
            }
        }
    }
    fact
}
}
