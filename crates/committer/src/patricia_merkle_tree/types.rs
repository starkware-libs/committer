use crate::hash::types::{HashFunction, HashOutput, PedersenHashFunction};
use crate::patricia_merkle_tree::filled_node::NodeData;
use crate::types::Felt;

use crate::patricia_merkle_tree::filled_node::LeafData;

pub(crate) trait TreeHashFunction<L: LeafDataTrait, H: HashFunction> {
    /// Computes the hash of given node data.
    fn compute_node_hash(node_data: NodeData<L>) -> HashOutput;
}

pub(crate) struct TreeHashFunctionTestingImpl;

impl TreeHashFunction<LeafData, PedersenHashFunction> for TreeHashFunctionTestingImpl {
    fn compute_node_hash(node_data: NodeData<LeafData>) -> HashOutput {
        match node_data {
            // testing implementation
            NodeData::Binary(_) => todo!(),
            NodeData::Edge(_) => todo!(),
            NodeData::Leaf(leaf_data) => match leaf_data {
                LeafData::StorageValue(_) => todo!(),
                LeafData::CompiledClassHash(compiled_class_hash) => {
                    HashOutput(compiled_class_hash.0)
                }
                LeafData::StateTreeTuple { .. } => {
                    todo!()
                }
            },
        }
    }
}

// TODO(Amos, 01/05/2024): Implement types for NodeIndex, EdgePath, EdgePathLength
#[allow(dead_code)]
pub(crate) struct NodeIndex(pub Felt);

#[allow(dead_code)]
pub(crate) struct EdgePath(pub Felt);

#[allow(dead_code)]
pub(crate) struct EdgePathLength(pub u8);

#[allow(dead_code)]
pub(crate) struct PathToBottom {
    pub path: EdgePath,
    pub length: EdgePathLength,
}

#[allow(dead_code)]
pub(crate) struct EdgeData {
    bottom_hash: HashOutput,
    path_to_bottom: PathToBottom,
}

pub(crate) trait LeafDataTrait {
    /// Returns true if leaf is empty.
    fn is_empty(&self) -> bool;
}
