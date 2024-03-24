use starknet_api::core::{ClassHash, Nonce};

use crate::patricia_merkle_tree::types::{Leaf, PathToBottom};
use crate::{hash::types::HashOutput, types::Felt};

#[allow(dead_code)]
pub(crate) enum FilledNode<L: Leaf> {
    Binary { data: BinaryData, hash: HashOutput },
    Edge { data: EdgeData<L>, hash: HashOutput },
    Leaf(L),
}

#[allow(dead_code)]
pub(crate) struct BinaryData {
    left_hash: HashOutput,
    right_hash: HashOutput,
}

#[allow(dead_code)]
pub(crate) struct EdgeData<L: Leaf> {
    bottom_value: BottomData<L>,
    path_to_bottom: PathToBottom,
}

#[allow(dead_code)]
pub(crate) enum BottomData<L: Leaf> {
    BottomBinaryData(BinaryData),
    BottomLeafData(L),
}

#[allow(dead_code)]
pub(crate) enum LeafEnum {
    StorageValue(Felt),
    CompiledClassHash(Felt),
    StateTreeValue {
        class_hash: ClassHash,
        contract_state_root_hash: Felt,
        nonce: Nonce,
    },
}
