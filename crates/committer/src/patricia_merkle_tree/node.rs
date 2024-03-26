use starknet_api::core::{ClassHash, Nonce};

use crate::types::Felt;

use crate::hash::types::{HashFunction, HashOutput};

use super::errors::TreeHashComputationError;

pub(crate) struct NodeIndex(pub Felt);

#[allow(dead_code)]
pub(crate) struct EdgePath(pub Felt);

#[allow(dead_code)]
pub(crate) struct EdgePathLength(pub Felt);

#[allow(dead_code)]
pub(crate) enum NodeValue {
    Binary(Option<BinaryValue>),
    Edge {
        bottom_value: Option<BottomValue>,
        path_to_bottom: PathToBottom,
    },
    Leaf(LeafValue),
    Sibling,
}

#[allow(dead_code)]
pub(crate) enum BinaryValue {
    Left(HashOutput),
    Right(HashOutput),
}

#[allow(dead_code)]
pub(crate) enum BottomValue {
    BottomBinaryValue(BinaryValue),
    BottomLeafValue(LeafValue),
}

#[allow(dead_code)]
pub(crate) struct PathToBottom {
    pub path: EdgePath,
    pub length: EdgePathLength,
}

#[allow(dead_code)]
pub(crate) enum LeafValue {
    StorageKey(Felt),
    CompiledClassHash(Felt),
    StateTreeValue {
        class_hash: ClassHash,
        contract_state_root_hash: Felt,
        nonce: Nonce,
    },
}

pub(crate) trait Node<H: HashFunction> {
    /// Traverses sub-tree and computes & sets all nodes' hashes and values, if possible.
    /// If successful or if hash and value of node are already set - returns hash.
    fn compute_and_set_hash_and_value_recursively(
        &mut self,
    ) -> Result<HashOutput, TreeHashComputationError>;

    /// Computes and returns hash of node, if possible.
    fn compute_hash(node: Option<&Self>) -> Result<HashOutput, TreeHashComputationError>;

    /// Returns node's value. If value was not set some fields may be empty.
    fn get_value(&self) -> NodeValue;

    /// Returns node's hash if it was set.
    fn get_hash(&self) -> Option<HashOutput>;
}
