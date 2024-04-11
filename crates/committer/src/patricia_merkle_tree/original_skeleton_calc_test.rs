use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::original_skeleton_calc::LeafData;
use crate::patricia_merkle_tree::original_skeleton_node::OriginalSkeletonNode;
use crate::patricia_merkle_tree::types::{EdgePath, EdgePathLength, NodeIndex, PathToBottom};
use crate::types::Felt;
use pretty_assertions::assert_eq;
use rstest::rstest;
use std::collections::HashMap;

use crate::patricia_merkle_tree::types::TreeHeight;
use crate::storage::storage_trait::{StorageKey, StoragePrefix, StorageValue};

use super::OriginalSkeletonTreeImpl;
use super::SubTree;

#[rstest]
// This test assumes for simplicity that hash is addition (i.e hash(a,b) = a + b).
///
///                 Old tree structure:
///
///                             50
///                           /   \
///                         30     20
///                        /  \     \
///                       17  13     \
///                      /  \   \     \
///                     8    9  11     15
///
///                   Modified leaves indices: [8, 10, 13]
///
///                   Expected skeleton:
///
///                             B
///                           /   \
///                          B     E
///                         / \     \
///                        B   E     \
///                       / \   \     \
///                      NZ  9  11    15
///
///

#[case::simple_tree_of_height_3(
    HashMap::from([
    (create_patricia_key(8), StorageValue(create_32_bytes_entry(8))),
    (create_patricia_key(9), StorageValue(create_32_bytes_entry(9))),
    (create_patricia_key(11), StorageValue(create_32_bytes_entry(11))),
    (create_patricia_key(15), StorageValue(create_32_bytes_entry(15))),
    (create_patricia_key(8 + 9), create_binary_val(8, 9)),
    (create_patricia_key(11 + 1 + 1), create_edge_val(11, 1, 1)),
    (create_patricia_key(17 + 13),
    create_binary_val(17, 13)),
    (create_patricia_key(15 + 3 + 2), create_edge_val(15, 3, 2)),
    (create_patricia_key(30 + 20), create_binary_val(30, 20)),
    ]),
    create_modifications(vec![(8, 4), (10, 3), (13, 2)]),
    HashOutput(Felt::from(50_u128)),
    HashMap::from([
        (NodeIndex::from(1), OriginalSkeletonNode::Binary),
        (NodeIndex::from(2), OriginalSkeletonNode::Binary),
        (NodeIndex::from(3), OriginalSkeletonNode::Edge { path_to_bottom: PathToBottom {
            path: EdgePath(Felt::from(3_u128)), length: EdgePathLength(2)
        } }),
        (NodeIndex::from(4), OriginalSkeletonNode::Binary),
        (NodeIndex::from(5), OriginalSkeletonNode::Edge { path_to_bottom: PathToBottom {
            path: EdgePath(Felt::from(1_u128)), length: EdgePathLength(1)
        } }),
        (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(9_u128))
        )),
        (NodeIndex::from(15), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(15_u128))
        )),
        (NodeIndex::from(11), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(11_u128))
        )),
    ]),
    TreeHeight(3)
)]
///                 Old tree structure:
///
///                             29
///                           /    \
///                         13      16
///                        /      /    \
///                       12      5     11
///                      /  \      \    /  \
///                     10   2      3   4   7
///
///                   Modified leaves indices: [8, 11, 13]
///
///                   Expected skeleton:
///
///                             B
///                           /   \
///                         E      B
///                        /     /    \
///                       B      E     E
///                      /  \     \     \
///                     NZ   2     NZ    NZ
///

#[case::another_simple_tree_of_height_3(
    HashMap::from([
    (create_patricia_key(10), StorageValue(create_32_bytes_entry(10))),
    (create_patricia_key(2), StorageValue(create_32_bytes_entry(2))),
    (create_patricia_key(3), StorageValue(create_32_bytes_entry(3))),
    (create_patricia_key(4), StorageValue(create_32_bytes_entry(4))),
    (create_patricia_key(7), StorageValue(create_32_bytes_entry(7))),
    (create_patricia_key(10 + 2), create_binary_val(10, 2)),
    (create_patricia_key(3 + 1 + 1), create_edge_val(3, 1, 1)),
    (create_patricia_key(4 + 7), create_binary_val(4, 7)),
    (create_patricia_key(12 + 1), create_edge_val(12, 0, 1)),
    (create_patricia_key(5 + 11), create_binary_val(5, 11)),
    (create_patricia_key(13 + 16), create_binary_val(13, 16)),
    ]),
    create_modifications(vec![(8, 5), (11, 1), (13, 3)]),
    HashOutput(Felt::from(29_u128)),
    HashMap::from([
        (NodeIndex::from(1), OriginalSkeletonNode::Binary),
        (NodeIndex::from(2), OriginalSkeletonNode::Edge {
            path_to_bottom: PathToBottom { path: EdgePath(Felt::ZERO), length: EdgePathLength(1) }
        }),
        (NodeIndex::from(3), OriginalSkeletonNode::Binary),
        (NodeIndex::from(4), OriginalSkeletonNode::Binary),
        (NodeIndex::from(6), OriginalSkeletonNode::Edge {
            path_to_bottom: PathToBottom {
                path: EdgePath(Felt::from(1_u128)), length: EdgePathLength(1)
            }
        }),
        (NodeIndex::from(7), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(11_u128))
        )),
        (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(2_u128))
        ))
    ]),
    TreeHeight(3)
)]
///                  Old tree structure:
///
///                             116
///                           /     \
///                         26       90
///                        /      /     \
///                       /      25      65
///                      /        \     /  \
///                     24         \   6   59
///                    /  \         \  /  /  \
///                   11  13       20  5  19 40
///
///                   Modified leaves indices: [18, 25, 29, 30]
///
///                   Expected skeleton:
///
///                              B
///                           /     \
///                         E        B
///                        /      /     \
///                       /      E       B
///                      /        \     /  \
///                     24         \   E    B
///                                 \  /     \
///                                 20 5     40
///
#[case::tree_of_height_4_with_long_edge(
    HashMap::from([
    (create_patricia_key(11), StorageValue(create_32_bytes_entry(11))),
    (create_patricia_key(13), StorageValue(create_32_bytes_entry(13))),
    (create_patricia_key(20), StorageValue(create_32_bytes_entry(20))),
    (create_patricia_key(5), StorageValue(create_32_bytes_entry(5))),
    (create_patricia_key(19), StorageValue(create_32_bytes_entry(19))),
    (create_patricia_key(40), StorageValue(create_32_bytes_entry(40))),
    (create_patricia_key(11 + 13), create_binary_val(11, 13)),
    (create_patricia_key(5 + 1), create_edge_val(5, 0, 1)),
    (create_patricia_key(19 + 40), create_binary_val(19, 40)),
    (create_patricia_key(20 + 3 + 2), create_edge_val(20, 3, 2)),
    (create_patricia_key(6 + 59), create_binary_val(6, 59)),
    (create_patricia_key(24 + 2), create_edge_val(24, 0, 2)),
    (create_patricia_key(25 + 65), create_binary_val(25, 65)),
    (create_patricia_key(26 + 90), create_binary_val(26, 90)),
    ]),
    create_modifications(vec![(18, 5), (25, 1), (29, 15), (30, 3)]),
    HashOutput(Felt::from(116_u128)),
    HashMap::from([
        (NodeIndex::from(1), OriginalSkeletonNode::Binary),
        (NodeIndex::from(2), OriginalSkeletonNode::Edge {
            path_to_bottom: PathToBottom { path: EdgePath(Felt::ZERO), length: EdgePathLength(2) } 
        }),
        (NodeIndex::from(3), OriginalSkeletonNode::Binary),
        (NodeIndex::from(6), OriginalSkeletonNode::Edge {
            path_to_bottom: PathToBottom { path: EdgePath(Felt::from(3_u128)),
                                           length: EdgePathLength(2) }
        }),
        (NodeIndex::from(7), OriginalSkeletonNode::Binary),
        (NodeIndex::from(8), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(24_u128))
        )),
        (NodeIndex::from(14), OriginalSkeletonNode::Edge {
            path_to_bottom: PathToBottom {path: EdgePath(Felt::ZERO), length: EdgePathLength(1)}
        }),
        (NodeIndex::from(15), OriginalSkeletonNode::Binary),
        (NodeIndex::from(27), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(20_u128))
        )),
        (NodeIndex::from(28), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(5_u128))
        )),
        (NodeIndex::from(31), OriginalSkeletonNode::LeafOrBinarySibling(
            HashOutput(Felt::from(40_u128))
        )),

    ]),
    TreeHeight(4)
)]
fn test_fetch_nodes(
    #[case] storage: HashMap<StorageKey, StorageValue>,
    #[case] leaf_modifications: HashMap<NodeIndex, LeafData>,
    #[case] root_hash: HashOutput,
    #[case] expected_nodes: HashMap<NodeIndex, OriginalSkeletonNode<LeafData>>,
    #[case] tree_height: TreeHeight,
) {
    let mut skeleton_tree = OriginalSkeletonTreeImpl {
        nodes: HashMap::new(),
        leaf_modifications,
        tree_height,
    };
    let mut sorted_leaf_indices: Vec<NodeIndex> =
        skeleton_tree.leaf_modifications.keys().copied().collect();
    sorted_leaf_indices.sort();
    let subtrees = vec![SubTree {
        sorted_leaf_indices: &sorted_leaf_indices,
        root_index: NodeIndex(Felt::ONE),
        root_hash: StorageKey::from(root_hash.0),
    }];
    assert!(skeleton_tree.fetch_nodes(subtrees, storage).is_ok());
    assert_eq!(&skeleton_tree.nodes, &expected_nodes);
}

fn create_32_bytes_entry(simple_val: u8) -> Vec<u8> {
    let mut res = vec![0; 31];
    res.push(simple_val);
    res
}

fn create_patricia_key(val: u8) -> StorageKey {
    StorageKey(create_32_bytes_entry(val)).with_prefix(StoragePrefix::PatriciaNode)
}
#[allow(dead_code)]
fn create_binary_val(left: u8, right: u8) -> StorageValue {
    StorageValue(
        (create_32_bytes_entry(left)
            .into_iter()
            .chain(create_32_bytes_entry(right)))
        .collect(),
    )
}
#[allow(dead_code)]
fn create_edge_val(hash: u8, path: u8, length: u8) -> StorageValue {
    StorageValue(
        create_32_bytes_entry(hash)
            .into_iter()
            .chain(create_32_bytes_entry(path))
            .chain([length])
            .collect(),
    )
}

fn create_modifications(modifications: Vec<(u128, u128)>) -> HashMap<NodeIndex, LeafData> {
    modifications
        .into_iter()
        .map(|(idx, val)| {
            (
                NodeIndex::from(idx),
                LeafData::StorageValue(Felt::from(val)),
            )
        })
        .collect()
}
