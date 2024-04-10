use std::collections::HashMap;

use crate::hash::hash_trait::HashOutput;
use crate::hash::pedersen::PedersenHashFunction;
use crate::patricia_merkle_tree::filled_node::{ClassHash, LeafData};
use crate::patricia_merkle_tree::filled_tree::FilledTree;
use crate::patricia_merkle_tree::types::{
    EdgePath, EdgePathLength, NodeIndex, PathToBottom, TreeHashFunctionImpl,
};
use crate::patricia_merkle_tree::updated_skeleton_node::UpdatedSkeletonNode;
use crate::patricia_merkle_tree::updated_skeleton_tree::{
    UpdatedSkeletonTree, UpdatedSkeletonTreeImpl,
};
use crate::types::Felt;

#[test]
/// This test is a sanity test for computing the root hash of the patricia merkle tree with a single node that is a leaf with hash==1.
fn test_filled_tree_sanity() {
    let mut skeleton_tree: HashMap<NodeIndex, UpdatedSkeletonNode<LeafData>> = HashMap::new();
    skeleton_tree.insert(
        NodeIndex::root_index(),
        UpdatedSkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::ONE))),
    );
    let updated_skeleton_tree = UpdatedSkeletonTreeImpl { skeleton_tree };
    let root_hash = updated_skeleton_tree
        .compute_filled_tree::<PedersenHashFunction, TreeHashFunctionImpl<PedersenHashFunction>>()
        .unwrap()
        .get_root_hash()
        .unwrap();
    assert_eq!(root_hash, HashOutput(Felt::ONE), "Root hash mismatch");
}

#[test]
/// This test is a small test for testing the root hash computation of the patricia merkle tree.
/// The tree structure & results were computed seperately and tested for regression.
fn test_small_filled_tree() {
    let mut skeleton_tree: HashMap<NodeIndex, UpdatedSkeletonNode<LeafData>> = HashMap::new();
    skeleton_tree.insert(NodeIndex::root_index(), UpdatedSkeletonNode::Binary);
    skeleton_tree.insert(
        NodeIndex(Felt::TWO),
        UpdatedSkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::ZERO),
                length: EdgePathLength(1),
            },
        },
    );
    skeleton_tree.insert(
        NodeIndex(Felt::THREE),
        UpdatedSkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::from(15_u128)),
                length: EdgePathLength(4),
            },
        },
    );
    skeleton_tree.insert(NodeIndex(Felt::from(4_u128)), UpdatedSkeletonNode::Binary);
    skeleton_tree.insert(
        NodeIndex(Felt::from(8_u128)),
        UpdatedSkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::THREE),
                length: EdgePathLength(2),
            },
        },
    );
    skeleton_tree.insert(
        NodeIndex(Felt::from(9_u128)),
        UpdatedSkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::ZERO),
                length: EdgePathLength(2),
            },
        },
    );

    // The leaves are the compiled class hashes, with hash values of 1, 2, 3.
    skeleton_tree.insert(
        NodeIndex(Felt::from(35_u128)),
        UpdatedSkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::ONE))),
    );
    skeleton_tree.insert(
        NodeIndex(Felt::from(36_u128)),
        UpdatedSkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::TWO))),
    );
    skeleton_tree.insert(
        NodeIndex(Felt::from(63_u128)),
        UpdatedSkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::THREE))),
    );

    let updated_skeleton_tree = UpdatedSkeletonTreeImpl { skeleton_tree };

    let root_hash = updated_skeleton_tree
        .compute_filled_tree::<PedersenHashFunction, TreeHashFunctionImpl<PedersenHashFunction>>()
        .unwrap()
        .get_root_hash()
        .unwrap();

    // The expected root hash was computed separately.
    let expected_root_hash = HashOutput(
        Felt::from_hex("0xe8899e8c731a35f5e9ce4c4bc32aabadcc81c5cdcc1aeba74fa7509046c338").unwrap(),
    );
    assert_eq!(root_hash, expected_root_hash, "Root hash mismatch");
}
