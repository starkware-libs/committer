use std::collections::HashMap;

use crate::{
    hash::types::{HashOutput, PedersenHashFunction},
    patricia_merkle_tree::{
        filled_node::{ClassHash, LeafData},
        skeleton_node::SkeletonNode,
        skeleton_tree::{UpdatedSkeletonTree, UpdatedSkeletonTreeImpl},
        types::{EdgePath, EdgePathLength, NodeIndex, PathToBottom, TreeHashFunctionTestingImpl},
    },
    types::Felt,
};

use crate::patricia_merkle_tree::filled_tree::FilledTree;

#[test]
fn test_patricia_sanity() {
    // This test is a sanity test for computing the root hash of the patricia merkle tree with a single node that is a leaf with hash==1.
    let mut skeleton_tree: HashMap<NodeIndex, SkeletonNode<LeafData>> = HashMap::new();
    skeleton_tree.insert(
        NodeIndex::root_index(),
        SkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::from(1)))),
    );
    let updated_skeleton_tree = UpdatedSkeletonTreeImpl {
        skeleton_tree,
        hash_function: PedersenHashFunction,
        tree_hash_function: TreeHashFunctionTestingImpl,
    };
    let root_hash = updated_skeleton_tree
        .compute_filled_tree()
        .unwrap()
        .get_root_hash();
    assert_eq!(root_hash, HashOutput(Felt::ONE), "Root hash mismatch");
}

#[test]
fn test_patricia_small() {
    // This test is a small test for testing the root hash computation of the patricia merkle tree.
    // The test & result are taken from the python test test_patricia.
    let mut skeleton_tree: HashMap<NodeIndex, SkeletonNode<LeafData>> = HashMap::new();
    skeleton_tree.insert(NodeIndex::root_index(), SkeletonNode::Binary);
    skeleton_tree.insert(
        NodeIndex(Felt::from(2)),
        SkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::ZERO),
                length: EdgePathLength(1),
            },
        },
    );
    skeleton_tree.insert(
        NodeIndex(Felt::from(3)),
        SkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::from(15)),
                length: EdgePathLength(4),
            },
        },
    );
    skeleton_tree.insert(NodeIndex(Felt::from(4)), SkeletonNode::Binary);
    skeleton_tree.insert(
        NodeIndex(Felt::from(8)),
        SkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::from(3)),
                length: EdgePathLength(2),
            },
        },
    );
    skeleton_tree.insert(
        NodeIndex(Felt::from(9)),
        SkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::ZERO),
                length: EdgePathLength(2),
            },
        },
    );
    //The leaves are the compiled class hashes, with hash values of 1, 2, 3.
    skeleton_tree.insert(
        NodeIndex(Felt::from(35)),
        SkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::from(1)))),
    );
    skeleton_tree.insert(
        NodeIndex(Felt::from(36)),
        SkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::from(2)))),
    );
    skeleton_tree.insert(
        NodeIndex(Felt::from(63)),
        SkeletonNode::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::from(3)))),
    );

    //TODO: build the skeleton tree
    let updated_skeleton_tree = UpdatedSkeletonTreeImpl {
        skeleton_tree,
        hash_function: PedersenHashFunction,
        tree_hash_function: TreeHashFunctionTestingImpl,
    };
    let root_hash = updated_skeleton_tree
        .compute_filled_tree()
        .unwrap()
        .get_root_hash();

    // The expected root hash taken from the python test.
    let expected_root_hash = HashOutput(
        Felt::from_hex("0xe8899e8c731a35f5e9ce4c4bc32aabadcc81c5cdcc1aeba74fa7509046c338").unwrap(),
    );
    assert_eq!(root_hash, expected_root_hash, "Root hash mismatch");
}
