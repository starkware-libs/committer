use crate::hash::types::{HashFunction, HashOutput};
use crate::patricia_merkle_tree::filled_node::NodeData;
use crate::types::Felt;

pub(crate) trait TreeHashFunction<L: LeafDataTrait, H: HashFunction> {
    /// Computes the hash of given input.
    fn compute_node_hash(node_data: NodeData<L>) -> HashOutput;
    // async fn compute_leaf_hash(leaf_data: L) -> HashOutput;
}

// TODO(Amos, 01/05/2024): Implement types for NodeIndex, EdgePath, EdgePathLength
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct NodeIndex(pub Felt);

impl NodeIndex {
    pub(crate) fn root_index() -> NodeIndex {
        NodeIndex(Felt::ONE)
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct EdgePath(pub Felt);

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct EdgePathLength(pub u16);

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct PathToBottom {
    pub path: EdgePath,
    pub length: EdgePathLength,
}

pub(crate) trait LeafDataTrait {
    /// Returns true if leaf is empty.
    fn is_empty(&self) -> bool;
    // fn compute_leaf_hash(&self) -> HashOutput;
}
