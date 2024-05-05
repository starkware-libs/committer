use rstest::rstest;
use std::collections::HashMap;
use pretty_assertions::assert_eq;

use crate::block_committer::input::
{ContractAddress, ContractState, Input, StarknetStorageKey, StarknetStorageValue, StateDiff};
use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::node::{ClassHash, Nonce};
use crate::patricia_merkle_tree::node_data::inner_node::{EdgePath, EdgePathLength, PathToBottom};
use crate::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use crate::patricia_merkle_tree::original_skeleton_tree::create_tree::OriginalSkeletonTreeImpl;
use crate::patricia_merkle_tree::original_skeleton_tree::create_tree::create_tree_test::{create_binary_entry, create_edge_entry, create_32_bytes_entry};
use crate::patricia_merkle_tree::original_skeleton_tree::node::OriginalSkeletonNode;
use crate::patricia_merkle_tree::types::{NodeIndex, TreeHeight};
use crate::storage::map_storage::MapStorage;

use super::OriginalSkeletonForest;

// This test assumes for simplicity that hash is addition (i.e hash(a,b) = a + b).
///                                Old forest structure:
///                                 Global tree:
/// 
///                                     254
///                                    /   \
///                                   154  100
///                                  /       \   
///                                 *         98
///                                /         /  \
///                               152       44   54
///                               
///                          Modified leaves (full) indices: [9, 11, 13, 14]
/// 
/// 
/// 
///     Contracts #3, #5, #6:                        Contract #1:          
/// 
///               29                                          55
///             /    \                                      /    \
///           13      16                                  35      20
///          /      /    \                               /  \       \
///         12      5     11                            17  18       *
///        /  \      \   /  \                          / \   \        \
///       10   2      3  4   7                        8   9  16       15

///   Modified leaves (full) indices: [8, 11, 13]  ##  Modified leaves (full) indices: [8, 10, 13]

///                             Expected skeleton forest:
///                                 Global tree:
/// 
///                                      B
///                                    /   \
///                                   E     E
///                                  /       \   
///                                 *         B
///                                /         /  \
///                               152       NZ   54
/// 
///      Contracts #3, #5, #6:                        Contract #1:          


///              B                                           B
///            /   \                                       /   \
///          E      B                                     B     E
///         /     /    \                                 / \     \
///        B      E     E                               B   E     *
///       /  \     \     \                             / \   \     \
///      NZ   2     NZ    NZ                          NZ  9  16    15
///



#[rstest]
#[case(
    Input {
        storage: HashMap::from([
            create_binary_entry(8, 9),
            create_edge_entry(16, 1, 1),
            create_binary_entry(17, 18),
            create_edge_entry(15, 3, 2),
            create_binary_entry(35, 20),
            create_binary_entry(10, 2),
            create_edge_entry(3, 1, 1),
            create_binary_entry(4, 7),
            create_edge_entry(12, 0, 1),
            create_binary_entry(5, 11),
            create_binary_entry(13, 16),
            create_edge_entry(152, 0, 2),
            create_binary_entry(44, 54),
            create_edge_entry(98, 1, 1),
            create_binary_entry(154, 100),
        ]),
        state_diff: StateDiff {
            address_to_class_hash: HashMap::new(),
            address_to_nonce: HashMap::new(),
            class_hash_to_compiled_class_hash: HashMap::new(),
            current_contract_state_leaves: create_contract_leaves(&[
                (3, 29),
                (5, 29),
                (6, 29),
                (1, 55),
            ]),
            storage_updates: create_storage_updates(&[
                (3, &[8, 11, 13]),
                (5, &[8, 11, 13]),
                (6, &[8, 11, 13]),
                (1, &[8, 10, 13]),
            ]),
        },
        tree_height: TreeHeight(3),
        global_tree_root_hash: HashOutput(Felt::from(254_u128)),
        classes_tree_root_hash: HashOutput(Felt::ZERO),
    }, OriginalSkeletonForest{
        global_state_tree: OriginalSkeletonTreeImpl {
            nodes: HashMap::from([
                (NodeIndex::from(1), OriginalSkeletonNode::Binary),
                (NodeIndex::from(2), OriginalSkeletonNode::Edge { 
                    path_to_bottom: PathToBottom {
                        path: EdgePath(Felt::ZERO),
                        length: EdgePathLength(2) }
                    }
                ),
                (NodeIndex::from(3), OriginalSkeletonNode::Edge { 
                    path_to_bottom: PathToBottom {
                        path: EdgePath(Felt::ONE),
                        length: EdgePathLength(1) }
                    }
                ),
                (NodeIndex::from(7), OriginalSkeletonNode::Binary),
                (NodeIndex::from(8), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(152_u128)))),
                (NodeIndex::from(15), OriginalSkeletonNode::LeafOrBinarySibling(HashOutput(Felt::from(54_u128))))
            ]),
            tree_height: TreeHeight(3) },
        contract_states: HashMap::from([
            (
                ContractAddress(Felt::from(1_u128)),
                OriginalSkeletonTreeImpl { nodes: HashMap::from([
                    (NodeIndex::from(1), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(2), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(3), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::from(3_u128)),
                            length: EdgePathLength(2),
                        },
                    }),
                    (NodeIndex::from(4), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(5), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::from(1_u128)),
                            length: EdgePathLength(1)
                        }
                    }),
                    (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(9_u128))
                    )),
                    (NodeIndex::from(15), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(15_u128))
                    )),
                    (NodeIndex::from(11), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(16_u128))
                    )),
                ]),
                    tree_height: TreeHeight(3) }
            ),
            (
                ContractAddress(Felt::from(3_u128)),
                OriginalSkeletonTreeImpl { nodes: HashMap::from([
                    (NodeIndex::from(1), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(2), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::ZERO),
                            length: EdgePathLength(1)
                        }
                    }),
                    (NodeIndex::from(3), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(4), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(6), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::from(1_u128)),
                            length: EdgePathLength(1)
                        }
                    }),
                    (NodeIndex::from(7), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(11_u128))
                    )),
                    (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(2_u128))
                    ))
                ]),
                tree_height: TreeHeight(3)
                }
            ),
            (
                ContractAddress(Felt::from(5_u128)),
                OriginalSkeletonTreeImpl { nodes: HashMap::from([
                    (NodeIndex::from(1), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(2), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::ZERO),
                            length: EdgePathLength(1)
                        }
                    }),
                    (NodeIndex::from(3), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(4), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(6), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::from(1_u128)),
                            length: EdgePathLength(1)
                        }
                    }),
                    (NodeIndex::from(7), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(11_u128))
                    )),
                    (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(2_u128))
                    ))
                ]),
                tree_height: TreeHeight(3)
                }
            ),
            (
                ContractAddress(Felt::from(6_u128)),
                OriginalSkeletonTreeImpl { nodes: HashMap::from([
                    (NodeIndex::from(1), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(2), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::ZERO),
                            length: EdgePathLength(1)
                        }
                    }),
                    (NodeIndex::from(3), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(4), OriginalSkeletonNode::Binary),
                    (NodeIndex::from(6), OriginalSkeletonNode::Edge {
                        path_to_bottom: PathToBottom {
                            path: EdgePath(Felt::from(1_u128)),
                            length: EdgePathLength(1)
                        }
                    }),
                    (NodeIndex::from(7), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(11_u128))
                    )),
                    (NodeIndex::from(9), OriginalSkeletonNode::LeafOrBinarySibling(
                        HashOutput(Felt::from(2_u128))
                    ))
                ]),
                tree_height: TreeHeight(3)
                }
            )

            ]),
        leaf_data: std::marker::PhantomData,
        }


    
)]
fn test_create_original_skeleton_forest(
    #[case] input: Input,
    #[case] expected_forest: OriginalSkeletonForest<
    LeafDataImpl,
    OriginalSkeletonTreeImpl
    >) {
        let actual_forest: OriginalSkeletonForest<
        LeafDataImpl,
        OriginalSkeletonTreeImpl> = OriginalSkeletonForest::create_original_skeleton_forest::<MapStorage>(input).unwrap();

        assert_eq!(actual_forest.global_state_tree, expected_forest.global_state_tree);
}



fn create_contract_leaves(leaves: &[(u8, u8)]) -> HashMap<ContractAddress, ContractState> {
    leaves
        .iter()
        .map(|(idx, root)| {
            (
                ContractAddress(Felt::from_bytes_be_slice(&create_32_bytes_entry(*idx))),
                ContractState {
                    nonce: Nonce(Felt::ZERO),
                    storage_root_hash: HashOutput(Felt::from_bytes_be_slice(
                        &create_32_bytes_entry(*root),
                    )),
                    class_hash: ClassHash(Felt::ZERO),
                },
            )
        })
        .collect()
}

fn create_storage_updates(
    updates: &[(u8, &[u8])],
) -> HashMap<ContractAddress, HashMap<StarknetStorageKey, StarknetStorageValue>> {
    updates
        .iter()
        .map(|(address, address_indices)| {
            (
                ContractAddress(Felt::from(u128::from(*address))),
                address_indices
                    .iter()
                    .map(|val| {
                        (
                            StarknetStorageKey(Felt::from(u128::from(*val))),
                            StarknetStorageValue(Felt::from(u128::from(*val))),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}
