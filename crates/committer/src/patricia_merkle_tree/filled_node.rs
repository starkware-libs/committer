use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::types::{EdgeData, LeafDataTrait};
use crate::storage::storage_trait::{StorageKey, StorageValue};
use crate::types::Felt;

use super::original_skeleton_tree::OriginalSkeletonTreeResult;
use super::types::{EdgePath, EdgePathLength, PathToBottom};

// TODO(Nimrod, 1/6/2024): Swap to starknet-types-core types once implemented.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ClassHash(pub Felt);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Nonce(pub Felt);

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
/// A node in a Patricia-Merkle tree which was modified during an update.
pub(crate) struct FilledNode<L: LeafDataTrait> {
    pub(crate) hash: HashOutput,
    pub(crate) data: NodeData<L>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
// A Patricia-Merkle tree node's data, i.e., the pre-image of its hash.
pub(crate) enum NodeData<L: LeafDataTrait> {
    Binary(BinaryData),
    Edge(EdgeData),
    Leaf(L),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BinaryData {
    pub(crate) left_hash: HashOutput,
    pub(crate) right_hash: HashOutput,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

#[allow(dead_code)]
impl FilledNode<LeafData> {
    pub(crate) fn deserialize(
        key: &StorageKey,
        value: &StorageValue,
    ) -> OriginalSkeletonTreeResult<Self> {
        // TODO(Nimrod, 30/4/2024): Compare to constant values once [PR #18] (https://github.com/starkware-libs/committer/pull/18) is merged.
        if value.0.len() == 64 {
            Ok(Self {
                hash: HashOutput(Felt::from_bytes_be_slice(&key.0)),
                data: NodeData::Binary(BinaryData {
                    left_hash: HashOutput(Felt::from_bytes_be_slice(&value.0[..32])),
                    right_hash: HashOutput(Felt::from_bytes_be_slice(&value.0[32..])),
                }),
            })
        }
        // TODO(Nimrod, 30/4/2024): Compare to constant values once [PR #18] (https://github.com/starkware-libs/committer/pull/18) is merged.
        else if value.0.len() == 65 {
            return Ok(Self {
                hash: HashOutput(Felt::from_bytes_be_slice(&key.0)),
                data: NodeData::Edge(EdgeData {
                    bottom_hash: HashOutput(Felt::from_bytes_be_slice(&value.0[..32])),
                    path_to_bottom: PathToBottom {
                        path: EdgePath(Felt::from_bytes_be_slice(&value.0[32..64])),
                        length: EdgePathLength(value.0[64]),
                    },
                }),
            });
        } else {
            todo!("Deserialize leaves.")
        }
    }
}
