use crate::hash::types::{HashFunction, HashInputPair, HashOutput, PedersenHashFunction};
use crate::patricia_merkle_tree::filled_node::{BinaryData, NodeData};
use crate::patricia_merkle_tree::types::TreeHashFunction;
use crate::patricia_merkle_tree::types::{
    EdgePath, EdgePathLength, NodeIndex, PathToBottom, TreeHashFunctionImpl,
};
use crate::types::Felt;
use rstest::rstest;
#[rstest]
#[case(1, 1, 1, 3)]
#[case(1, 0, 2, 4)]
#[case(0xDAD, 0xFEE, 12, 0xDADFEE)]
#[case(0xDEAFBEE, 0xBFF, 16, 0xDEAFBEE0BFF)]
fn test_compute_bottom_index(
    #[case] node_index: u128,
    #[case] path: u128,
    #[case] length: u8,
    #[case] expected: u128,
) {
    let bottom_index = NodeIndex::compute_bottom_index(
        NodeIndex(Felt::from(node_index)),
        PathToBottom {
            path: EdgePath(Felt::from(path)),
            length: EdgePathLength(length),
        },
    );
    let expected = NodeIndex(Felt::from(expected));
    assert_eq!(bottom_index, expected);
}

#[rstest]
#[case(Felt::ONE, Felt::TWO)]
#[case(Felt::from(0xBE_u128), Felt::from(0xA0BEE_u128))]
fn test_tree_hash_function_impl_binary_node(#[case] left_hash: Felt, #[case] right_hash: Felt) {
    let hash_output = TreeHashFunctionImpl::compute_node_hash(NodeData::Binary(BinaryData {
        left_hash: HashOutput(left_hash),
        right_hash: HashOutput(right_hash),
    }));
    assert_eq!(
        hash_output,
        PedersenHashFunction::compute_hash(HashInputPair(left_hash, right_hash))
    );
}

#[rstest]
#[case(Felt::ONE, Felt::TWO, 3)]
#[case(Felt::from(0xBE_u128), Felt::from(0xA0BEE_u128), 0xB)]
fn test_tree_hash_function_impl_edge_node(
    #[case] bottom_hash: Felt,
    #[case] edge_path: Felt,
    #[case] length: u8,
) {
    use crate::patricia_merkle_tree::types::EdgeData;

    let hash_output = TreeHashFunctionImpl::compute_node_hash(NodeData::Edge(EdgeData {
        bottom_hash: HashOutput(bottom_hash),
        path_to_bottom: PathToBottom {
            path: EdgePath(edge_path),
            length: EdgePathLength(length),
        },
    }));
    assert_eq!(
        hash_output,
        HashOutput(
            PedersenHashFunction::compute_hash(HashInputPair(bottom_hash, edge_path)).0
                + Felt::from(length)
        )
    );
}
