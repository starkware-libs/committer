use crate::{
    hash::types::{HashFunction, HashInputPair, HashOutput, PedersenHashFunction},
    patricia_merkle_tree::filled_node::{BinaryData, LeafData, NodeData},
    types::Felt,
};

use super::{EdgeData, PathToBottom, TreeHashFunction};

pub(crate) struct MockTreeHashFunctionImpl;

/// Mock implementation of TreeHashFunction for testing purposes.
impl TreeHashFunction<LeafData, PedersenHashFunction> for MockTreeHashFunctionImpl {
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
