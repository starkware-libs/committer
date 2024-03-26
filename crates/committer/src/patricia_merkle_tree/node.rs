use starknet_api::core::{ClassHash, Nonce};

use crate::types::Felt;

use crate::hash::types::{HashFunction, HashOutput};

use super::errors::NodeHashComputationError;

// TODO(Amos, 01/05/2024): Implement types for NodeIndex, EdgePath, EdgePathLength
#[allow(dead_code)]
pub(crate) struct NodeIndex(pub Felt);

#[allow(dead_code)]
pub(crate) struct EdgePath(pub Felt);

#[allow(dead_code)]
pub(crate) struct EdgePathLength(pub Felt);

#[allow(dead_code)]
pub(crate) enum NodeValue<LeafVal> {
    Binary(Option<BinaryValue>),
    Edge {
        bottom_value: Option<BottomValue>,
        path_to_bottom: PathToBottom,
    },
    Leaf(LeafVal),
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
    StorageValue(Felt),
    CompiledClassHash(Felt),
    StateTreeValue {
        class_hash: ClassHash,
        contract_state_root_hash: Felt,
        nonce: Nonce,
    },
}

pub(crate) trait Node<H: HashFunction, LeafVal> {
    /// Computes and sets sub-tree's hashes and values, if possible.
    /// If successful or if hash and value of node are already set - returns hash of node.
    fn compute_and_set_subtree_hashes_and_values(
        &mut self,
    ) -> Result<HashOutput, NodeHashComputationError>;

    /// Computes and returns hash of node, if value is set.
    /// If node is None - returns hash of empty tree.
    fn compute_hash(node: Option<&Self>) -> Result<HashOutput, NodeHashComputationError>;

    /// Returns node's value. If value was not set some fields may be empty.
    fn get_value(&self) -> NodeValue<LeafVal>;

    /// Returns node's hash if it was set.
    fn get_hash(&self) -> Option<HashOutput>;
}
