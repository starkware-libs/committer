use crate::hash::types::{HashFunction, HashInputPair, HashOutput, PedersenHashFunction};
use crate::patricia_merkle_tree::filled_node::{BinaryData, LeafData, NodeData};
use crate::types::Felt;

#[cfg(test)]
#[path = "types_test.rs"]
pub mod types_test;

pub(crate) trait TreeHashFunction<L: LeafDataTrait, H: HashFunction> {
    /// Computes the hash of given node data.
    fn compute_node_hash(node_data: NodeData<L>) -> HashOutput;
}

pub(crate) struct TreeHashFunctionImpl;

/// Implementation of TreeHashFunction.
// TODO(Aner, 11/4/25): Implement the function for LeafData::StorageValue and LeafData::StateTreeTuple
// TODO(Aner, 11/4/24): Verify the correctness of the implementation.
impl TreeHashFunction<LeafData, PedersenHashFunction> for TreeHashFunctionImpl {
    fn compute_node_hash(node_data: NodeData<LeafData>) -> HashOutput {
        match node_data {
            NodeData::Binary(BinaryData {
                left_hash,
                right_hash,
            }) => PedersenHashFunction::compute_hash(HashInputPair(left_hash.0, right_hash.0)),
            NodeData::Edge(EdgeData {
                bottom_hash: hash_output,
                path_to_bottom: PathToBottom { path, length },
            }) => HashOutput(
                PedersenHashFunction::compute_hash(HashInputPair(hash_output.0, path.0)).0
                    + Felt::from(length.0),
            ),
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct NodeIndex(pub Felt);

#[allow(dead_code)]
impl NodeIndex {
    pub(crate) fn root_index() -> NodeIndex {
        NodeIndex(Felt::ONE)
    }

    pub(crate) fn compute_bottom_index(
        index: NodeIndex,
        path_to_bottom: PathToBottom,
    ) -> NodeIndex {
        let PathToBottom { path, length } = path_to_bottom;
        NodeIndex(index.0 * Felt::TWO.pow(length.0) + path.0)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) struct EdgePath(pub Felt);

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) struct EdgePathLength(pub u8);

#[allow(dead_code)]
pub(crate) struct TreeHeight(pub u8);

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) struct PathToBottom {
    pub path: EdgePath,
    pub length: EdgePathLength,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) struct EdgeData {
    pub(crate) bottom_hash: HashOutput,
    pub(crate) path_to_bottom: PathToBottom,
}

pub(crate) trait LeafDataTrait {
    /// Returns true if leaf is empty.
    fn is_empty(&self) -> bool;
}
