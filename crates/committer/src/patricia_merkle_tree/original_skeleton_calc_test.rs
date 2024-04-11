use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::original_skeleton_calc::LeafData;
use crate::patricia_merkle_tree::original_skeleton_node::OriginalSkeletonNode;
use crate::patricia_merkle_tree::types::{EdgePath, EdgePathLength, NodeIndex, PathToBottom};
use crate::types::Felt;
use pretty_assertions::assert_eq;
use rstest::rstest;
use std::collections::HashMap;

use crate::patricia_merkle_tree::types::TreeHeight;
use crate::storage::storage_trait::{StorageKey, StorageValue};

use super::SkeletonTree;
use super::SubTree;

#[rstest]
// This test assumes for simplicity that hash is addition (i.e hash(a,b) = a + b).

/*                 Old tree structure:

                             50
                           /   \
                         30     20
                        /  \     \
                       17  13     \
                      /  \   \     \
                     8    9  11     15

                   Modified leaves indices: [8, 10, 13]

                   Expected skeleton:

                             50
                           /   \
                         30     20
                        /  \     \
                       17  13     \
                       / \   \     \
                          9  11    15

*/
#[case::simple_tree_of_height_3(
    HashMap::from([
    (StorageKey(create_32_bytes_entry(8)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(8))),
    (StorageKey(create_32_bytes_entry(9)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(9))),
    (StorageKey(create_32_bytes_entry(11)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(11))),
    (StorageKey(create_32_bytes_entry(15)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(15))),
    (StorageKey(create_32_bytes_entry(8 + 9)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(8).into_iter().chain(create_32_bytes_entry(9).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(11 + 1 + 1)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(11).into_iter().chain(create_32_bytes_entry(1).into_iter()).chain([1].into_iter()).collect())),
    (StorageKey(create_32_bytes_entry(17 + 13)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(17).into_iter().chain(create_32_bytes_entry(13).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(15 + 3 + 2)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(15).into_iter().chain(create_32_bytes_entry(3).into_iter())).chain([2].into_iter()).collect())),
    (StorageKey(create_32_bytes_entry(30 + 20)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(30).into_iter().chain(create_32_bytes_entry(20).into_iter())).collect())),
    ]),
    HashMap::from([
        (NodeIndex::from(8), LeafData::StorageValue(Felt::from(4_u128))),
        (NodeIndex::from(10), LeafData::StorageValue(Felt::from(3_u128))),
        (NodeIndex::from(13), LeafData::StorageValue(Felt::from(2_u128))),
    ]
    ),
    StorageKey(create_32_bytes_entry(50)),
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
        (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(9_u128)))),
        (NodeIndex::from(15), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(15_u128)))),
        (NodeIndex::from(11), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(11_u128)))),
    ]),
    TreeHeight(3)
)]
/*                 Old tree structure:

                             29
                           /    \
                         13      16
                        /      /    \
                       12      5     11
                      /  \      \    /  \
                     10   2      3   4   7

                   Modified leaves indices: [8, 11, 13]

                   Expected skeleton:

                             29
                           /    \
                         13      16
                        /      /    \
                       12      5     11
                      /  \      \
                          2
*/
#[case::another_simple_tree_of_height_3(
    HashMap::from([
    (StorageKey(create_32_bytes_entry(10)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(10))),
    (StorageKey(create_32_bytes_entry(2)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(2))),
    (StorageKey(create_32_bytes_entry(3)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(3))),
    (StorageKey(create_32_bytes_entry(4)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(4))),
    (StorageKey(create_32_bytes_entry(7)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(7))),
    (StorageKey(create_32_bytes_entry(10 + 2)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(10).into_iter().chain(create_32_bytes_entry(2).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(3 + 1 + 1)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(3).into_iter().chain(create_32_bytes_entry(1).into_iter()).chain([1].into_iter()).collect())),
    (StorageKey(create_32_bytes_entry(4 + 7)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(4).into_iter().chain(create_32_bytes_entry(7).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(12 + 1)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(12).into_iter().chain(create_32_bytes_entry(0).into_iter())).chain([1].into_iter()).collect())),
    (StorageKey(create_32_bytes_entry(5 + 11)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(5).into_iter().chain(create_32_bytes_entry(11).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(13 + 16)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(13).into_iter().chain(create_32_bytes_entry(16).into_iter())).collect())),
    ]),
    HashMap::from([
        (NodeIndex::from(8), LeafData::StorageValue(Felt::from(5_u128))),
        (NodeIndex::from(11), LeafData::StorageValue(Felt::from(1_u128))),
        (NodeIndex::from(13), LeafData::StorageValue(Felt::from(3_u128))),
    ]
    ),
    StorageKey(create_32_bytes_entry(29)),
    HashMap::from([
        (NodeIndex::from(1), OriginalSkeletonNode::Binary),
        (NodeIndex::from(2), OriginalSkeletonNode::Edge {path_to_bottom: PathToBottom {path: EdgePath(Felt::ZERO), length: EdgePathLength(1)}}),
        (NodeIndex::from(3), OriginalSkeletonNode::Binary),
        (NodeIndex::from(4), OriginalSkeletonNode::Binary),
        (NodeIndex::from(6), OriginalSkeletonNode::Edge {path_to_bottom: PathToBottom {path: EdgePath(Felt::from(1_u128)), length: EdgePathLength(1)}}),
        (NodeIndex::from(7), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(11_u128)))),
        (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(2_u128))))
    ]),
    TreeHeight(3)
)]
/*                 Old tree structure:

                             116
                           /     \
                        26        90
                        /      /     \
                       /      25      65
                      /        \     /  \
                     24         \   6   59
                    /  \         \  /  /  \
                   11  13       20  5  19 40

                   Modified leaves indices: [18, 25, 29, 30]

                   Expected skeleton:

                             116
                           /     \
                        26        90
                        /      /     \
                       /      25      65
                      /        \     /  \
                     24         \   6   59
                                 \  /     \
                                 20 5     40
*/
#[case::tree_of_height_4_with_long_edge(
    HashMap::from([
    (StorageKey(create_32_bytes_entry(11)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(11))),
    (StorageKey(create_32_bytes_entry(13)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(13))),
    (StorageKey(create_32_bytes_entry(20)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(20))),
    (StorageKey(create_32_bytes_entry(5)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(5))),
    (StorageKey(create_32_bytes_entry(19)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(19))),
    (StorageKey(create_32_bytes_entry(40)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(40))),
    (StorageKey(create_32_bytes_entry(11 + 13)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(11).into_iter().chain(create_32_bytes_entry(13).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(5 + 1)).with_patricia_prefix(),
    StorageValue(create_32_bytes_entry(5).into_iter().chain(create_32_bytes_entry(0).into_iter()).chain([1].into_iter()).collect())),
    (StorageKey(create_32_bytes_entry(19 + 40)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(19).into_iter().chain(create_32_bytes_entry(40).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(20 + 3 + 2)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(20).into_iter().chain(create_32_bytes_entry(3).into_iter())).chain([2].into_iter()).collect())),
    (StorageKey(create_32_bytes_entry(6 + 59)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(6).into_iter().chain(create_32_bytes_entry(59).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(24 + 2)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(24).into_iter().chain(create_32_bytes_entry(0).into_iter())).chain([2].into_iter()).collect())),
    (StorageKey(create_32_bytes_entry(25 + 65)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(25).into_iter().chain(create_32_bytes_entry(65).into_iter())).collect())),
    (StorageKey(create_32_bytes_entry(26 + 90)).with_patricia_prefix(),
    StorageValue((create_32_bytes_entry(26).into_iter().chain(create_32_bytes_entry(90).into_iter())).collect())),
    ]),
    HashMap::from([
        (NodeIndex::from(18), LeafData::StorageValue(Felt::from(5_u128))),
        (NodeIndex::from(25), LeafData::StorageValue(Felt::from(1_u128))),
        (NodeIndex::from(29), LeafData::StorageValue(Felt::from(14_u128))),
        (NodeIndex::from(30), LeafData::StorageValue(Felt::from(3_u128))),

    ]
    ),
    StorageKey(create_32_bytes_entry(116)),
    HashMap::from([
        (NodeIndex::from(1), OriginalSkeletonNode::Binary),
        (NodeIndex::from(2), OriginalSkeletonNode::Edge { path_to_bottom: PathToBottom {path: EdgePath(Felt::ZERO), length: EdgePathLength(2)}}),
        (NodeIndex::from(3), OriginalSkeletonNode::Binary),
        (NodeIndex::from(6), OriginalSkeletonNode::Edge { path_to_bottom: PathToBottom {path: EdgePath(Felt::from(3_u128)), length: EdgePathLength(2)}}),
        (NodeIndex::from(7), OriginalSkeletonNode::Binary),
        (NodeIndex::from(8), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(24_u128)))),
        (NodeIndex::from(14), OriginalSkeletonNode::Edge { path_to_bottom: PathToBottom {path: EdgePath(Felt::ZERO), length: EdgePathLength(1)}}),
        (NodeIndex::from(15), OriginalSkeletonNode::Binary),
        (NodeIndex::from(27), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(20_u128)))),
        (NodeIndex::from(28), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(5_u128)))),
        (NodeIndex::from(31), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(40_u128)))),

    ]),
    TreeHeight(4)
)]
fn test_fetch_nodes(
    #[case] storage: HashMap<StorageKey, StorageValue>,
    #[case] leaf_modifications: HashMap<NodeIndex, LeafData>,
    #[case] root_hash: StorageKey,
    #[case] expected_nodes: HashMap<NodeIndex, OriginalSkeletonNode<LeafData>>,
    #[case] tree_height: TreeHeight,
) {
    let mut skeleton_tree = SkeletonTree {
        nodes: HashMap::new(),
        leaf_modifications: &leaf_modifications,
        tree_height,
    };
    let mut sorted_leaf_indices: Vec<NodeIndex> =
        skeleton_tree.leaf_modifications.keys().copied().collect();
    sorted_leaf_indices.sort();
    let subtrees = vec![SubTree {
        sorted_leaf_indices: &sorted_leaf_indices,
        root_index: NodeIndex(Felt::ONE),
        root_hash,
    }];
    assert!(skeleton_tree.fetch_nodes(subtrees, storage).is_ok());
    assert_eq!(&skeleton_tree.nodes, &expected_nodes);
}

fn create_32_bytes_entry(simple_val: u8) -> Vec<u8> {
    let mut res = vec![0; 31];
    res.push(simple_val);
    res
}
