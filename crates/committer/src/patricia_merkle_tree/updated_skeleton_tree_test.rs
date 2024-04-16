use std::collections::HashMap;
use std::sync::Mutex;

use crate::hash::hash_trait::HashOutput;
use crate::hash::pedersen::PedersenHashFunction;
use crate::patricia_merkle_tree::filled_node::{
    BinaryData, ClassHash, FilledNode, LeafData, NodeData,
};
use crate::patricia_merkle_tree::filled_tree::FilledTree;
use crate::patricia_merkle_tree::types::{
    EdgeData, EdgePath, EdgePathLength, NodeIndex, PathToBottom, TreeHashFunctionImpl,
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
///                                i=1: binary
///                                /        \
///                        i=2: edge      i=3: edge
///                        l=1, p=0       l=4, p=15
///                      /                      \
///                 i=4: binary                  \
///                /           \                  \
///            i=8: edge    i=9: edge              \
///            l=2, p=3     l=2, p=0                \
///               \              /                   \
///                \            /                     \
///            i=35: leaf   i=36: leaf               i=63: leaf
///                  v=1          v=2                      v=3
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

    let filled_tree = updated_skeleton_tree
        .compute_filled_tree::<PedersenHashFunction, TreeHashFunctionImpl<PedersenHashFunction>>()
        .unwrap();
    let filled_tree_map = filled_tree.get_all_nodes();
    let root_hash = filled_tree.get_root_hash().unwrap();

    // The expected hash values were computed separately.
    let expected_root_hash = HashOutput(
        Felt::from_hex("0xe8899e8c731a35f5e9ce4c4bc32aabadcc81c5cdcc1aeba74fa7509046c338").unwrap(),
    );
    let expected_filled_tree_map = HashMap::from([
        (
            NodeIndex::root_index(),
            FilledNode {
                hash: HashOutput(
                    Felt::from_hex(
                        "0xe8899e8c731a35f5e9ce4c4bc32aabadcc81c5cdcc1aeba74fa7509046c338",
                    )
                    .unwrap(),
                ),
                data: NodeData::Binary(BinaryData {
                    left_hash: HashOutput(
                        Felt::from_hex(
                            "0x4e970ad06a06486b44fff5606c4f65486d31e05e323d65a618d4ef8cdf6d3a0",
                        )
                        .unwrap(),
                    ),
                    right_hash: HashOutput(
                        Felt::from_hex(
                            "0x2955a96b09495fb2ce4ed65cf679c54e54aefc2c6972d7f3042590000bb7543",
                        )
                        .unwrap(),
                    ),
                }),
            },
        ),
        (
            NodeIndex(Felt::TWO),
            FilledNode {
                hash: HashOutput(
                    Felt::from_hex(
                        "0x4e970ad06a06486b44fff5606c4f65486d31e05e323d65a618d4ef8cdf6d3a0",
                    )
                    .unwrap(),
                ),
                data: NodeData::Edge(EdgeData {
                    path_to_bottom: PathToBottom {
                        path: EdgePath(Felt::ZERO),
                        length: EdgePathLength(1),
                    },
                    bottom_hash: HashOutput(
                        Felt::from_hex(
                            "0x5d36a1ae900ef417a5696417dde9a0244b873522f40b552e4a60acde0991bc9",
                        )
                        .unwrap(),
                    ),
                }),
            },
        ),
        (
            NodeIndex(Felt::THREE),
            FilledNode {
                hash: HashOutput(
                    Felt::from_hex(
                        "0x2955a96b09495fb2ce4ed65cf679c54e54aefc2c6972d7f3042590000bb7543",
                    )
                    .unwrap(),
                ),
                data: NodeData::Edge(EdgeData {
                    path_to_bottom: PathToBottom {
                        path: EdgePath(Felt::from(15_u128)),
                        length: EdgePathLength(4),
                    },
                    bottom_hash: HashOutput(Felt::THREE),
                }),
            },
        ),
        (
            NodeIndex(Felt::from(4_u128)),
            FilledNode {
                hash: HashOutput(
                    Felt::from_hex(
                        "0x5d36a1ae900ef417a5696417dde9a0244b873522f40b552e4a60acde0991bc9",
                    )
                    .unwrap(),
                ),
                data: NodeData::Binary(BinaryData {
                    left_hash: HashOutput(
                        Felt::from_hex(
                            "0x582d984e4005c27b9c886cd00ec9a82ed5323aa629f6ea6b3ed7c0386ae6256",
                        )
                        .unwrap(),
                    ),
                    right_hash: HashOutput(
                        Felt::from_hex(
                            "0x39eb7b85bcc9deac314406d6b73154b09b008f8af05e2f58ab623f4201d0b88",
                        )
                        .unwrap(),
                    ),
                }),
            },
        ),
        (
            NodeIndex(Felt::from(8_u128)),
            FilledNode {
                hash: HashOutput(
                    Felt::from_hex(
                        "0x582d984e4005c27b9c886cd00ec9a82ed5323aa629f6ea6b3ed7c0386ae6256",
                    )
                    .unwrap(),
                ),
                data: NodeData::Edge(EdgeData {
                    path_to_bottom: PathToBottom {
                        path: EdgePath(Felt::THREE),
                        length: EdgePathLength(2),
                    },
                    bottom_hash: HashOutput(Felt::ONE),
                }),
            },
        ),
        (
            NodeIndex(Felt::from(9_u128)),
            FilledNode {
                hash: HashOutput(
                    Felt::from_hex(
                        "0x39eb7b85bcc9deac314406d6b73154b09b008f8af05e2f58ab623f4201d0b88",
                    )
                    .unwrap(),
                ),
                data: NodeData::Edge(EdgeData {
                    path_to_bottom: PathToBottom {
                        path: EdgePath(Felt::ZERO),
                        length: EdgePathLength(2),
                    },
                    bottom_hash: HashOutput(Felt::TWO),
                }),
            },
        ),
        (
            NodeIndex(Felt::from(35_u128)),
            FilledNode {
                hash: HashOutput(Felt::ONE),
                data: NodeData::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::ONE))),
            },
        ),
        (
            NodeIndex(Felt::from(36_u128)),
            FilledNode {
                hash: HashOutput(Felt::TWO),
                data: NodeData::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::TWO))),
            },
        ),
        (
            NodeIndex(Felt::from(63_u128)),
            FilledNode {
                hash: HashOutput(Felt::THREE),
                data: NodeData::Leaf(LeafData::CompiledClassHash(ClassHash(Felt::THREE))),
            },
        ),
    ]);
    assert_eq!(remove_mutex(filled_tree_map), expected_filled_tree_map);
    assert_eq!(root_hash, expected_root_hash, "Root hash mismatch");
}

fn remove_mutex(
    hash_map_in: &HashMap<NodeIndex, Mutex<FilledNode<LeafData>>>,
) -> HashMap<NodeIndex, FilledNode<LeafData>> {
    let mut hash_map_out: HashMap<NodeIndex, FilledNode<LeafData>> = HashMap::new();
    for (key, value) in hash_map_in.iter() {
        let value = value.lock().unwrap();
        hash_map_out.insert(*key, value.clone());
    }
    hash_map_out
}
