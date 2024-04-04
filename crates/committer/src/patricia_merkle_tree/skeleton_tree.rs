use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[allow(unused_imports)]
use crate::hash::types::{HashFunction, HashOutput};
use crate::patricia_merkle_tree::errors::SkeletonTreeError;
#[allow(unused_imports)]
use crate::patricia_merkle_tree::filled_node::{BinaryData, FilledNode};
use crate::patricia_merkle_tree::filled_tree::{self, FilledTree, FilledTreeImpl};
use crate::patricia_merkle_tree::skeleton_node::SkeletonNode;
use crate::patricia_merkle_tree::types::{LeafDataTrait, NodeIndex, TreeHashFunction};
use starknet_types_core::felt::Felt;
use starknet_types_core::hash::{self, StarkHash};

use super::filled_node::NodeData;

pub(crate) trait CurrentSkeletonTree<
    L: LeafDataTrait + std::clone::Clone,
    H: HashFunction,
    TH: TreeHashFunction<L, H>,
>
{
    /// Computes and returns updated skeleton tree.
    fn compute_updated_skeleton_tree(
        &self,
        index_to_updated_leaf: HashMap<NodeIndex, &SkeletonNode<L>>,
    ) -> Result<impl UpdatedSkeletonTree<L, H, TH>, SkeletonTreeError>;
}

pub(crate) trait UpdatedSkeletonTree<L: LeafDataTrait, H: HashFunction, TH: TreeHashFunction<L, H>>
{
    /// Computes and returns the filled tree.
    // TODO: add get_root impl
    // TODO: add get_preimage impl
    fn compute_filled_tree(&self) -> Result<impl FilledTree<L>, SkeletonTreeError>;
}

#[allow(dead_code)]
struct UpdatedSkeletonTreeImpl<
    L: LeafDataTrait + std::clone::Clone,
    H: HashFunction,
    TH: TreeHashFunction<L, H>,
> {
    root: SkeletonNode<L>,
    skeleton_tree: HashMap<NodeIndex, SkeletonNode<L>>,
    hash_function: H,
    tree_hash_function: TH,
}

#[allow(dead_code)]
impl<L: LeafDataTrait + std::clone::Clone, H: HashFunction, TH: TreeHashFunction<L, H>>
    UpdatedSkeletonTreeImpl<L, H, TH>
{
    fn get_root(&self) -> &SkeletonNode<L> {
        &self.root
    }
    fn get_sk_tree(&self) -> &HashMap<NodeIndex, SkeletonNode<L>> {
        &self.skeleton_tree
    }
    fn get_node(&self, index: NodeIndex) -> Option<&SkeletonNode<L>> {
        self.skeleton_tree.get(&index)
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn compute_filled_tree(
        &self,
        map: &HashMap<NodeIndex, SkeletonNode<L>>,
        index: NodeIndex,
        output_map: Arc<RwLock<HashMap<HashOutput, Box<FilledNode<L>>>>>,
    ) -> HashOutput {
        let node = &mut map.get(&index).unwrap();
        let node_hash = match node {
            SkeletonNode::Binary => {
                let left_index = NodeIndex(index.0 * TWO);
                let right_index = NodeIndex(left_index.0 + ONE);
                // rayon::scope(|s: &Scope<'_>| {
                //     s.spawn(|_s| {
                let left_hash = self.compute_filled_tree(map, left_index, Arc::clone(&output_map));
                // });
                // s.spawn(|_s| {
                let right_hash =
                    self.compute_filled_tree(map, right_index, Arc::clone(&output_map));
                //TODO:
                let hash_value = HashOutput(hash::Pedersen::hash(&left_hash.0, &right_hash.0));
                //let read_locked_map = output_map.read().expect("RwLock poisoned");
                let mut write_locked_map = output_map.write().expect("RwLock poisoned");
                write_locked_map.insert(
                    hash_value.clone(),
                    Box::new(FilledNode {
                        hash: hash_value.clone(),
                        data: NodeData::Binary(BinaryData {
                            left_hash,
                            right_hash,
                        }),
                    }),
                );
                // drop(read_locked_map);
                hash_value
            }
            SkeletonNode::Edge { path_to_bottom: _ } => todo!("Edge hash computation"),
            SkeletonNode::Sibling(hash_result) => {
                // let mut write_locked_map = output_map.write().expect("RwLock poisoned");
                // write_locked_map.insert(*hash_result, Box::new(hash_result));
                hash_result.clone()
            }
            SkeletonNode::Leaf(node_data) => {
                let mut write_locked_map = output_map.write().expect("RwLock poisoned");
                let tmp = TH::compute_node_hash(NodeData::Leaf(node_data.clone()));
                write_locked_map.insert(
                    tmp.clone(),
                    Box::new(FilledNode {
                        hash: tmp.clone(),
                        data: NodeData::Leaf(node_data.clone()),
                    }),
                );
                return tmp;
            }
            SkeletonNode::Empty => {
                unimplemented!("UpdatedSkeletonTree should not contain Empty nodes")
            }
        };
        node_hash
        // if node.is_leaf {
        //     let mut write_locked_map = output_map.write().expect("RwLock poisoned");
        //     write_locked_map.insert(index, Mutex::new(node.hash_value.unwrap()));
        //     return node.hash_value.unwrap();
        // }
        // let mut left_value: Felt = Default::default();
        // let mut right_value: Felt = Default::default();
        // let left_index = index * TWO;
        // let right_index = left_index + ONE;
        // // rayon::scope(|s: &Scope<'_>| {
        // //     s.spawn(|_s| {
        // left_value = compute_filled_tree(map, left_index, Arc::clone(&output_map));
        // // });
        // // s.spawn(|_s| {
        // right_value = compute_filled_tree(map, right_index, Arc::clone(&output_map));
        //     });
        // });
        // if !node.is_path {
        //     let hash_value = hash::Pedersen::hash(&left_value, &right_value);
        //     //let read_locked_map = output_map.read().expect("RwLock poisoned");
        //     let mut write_locked_map = output_map.write().expect("RwLock poisoned");
        //     write_locked_map.insert(index, Mutex::new(hash_value));
        //     // drop(read_locked_map);
        //     hash_value
        // } else {
        //     todo!("Path hash computation")
        // }
    }
}

impl<
        L: LeafDataTrait + std::fmt::Debug + std::clone::Clone,
        H: HashFunction,
        TH: TreeHashFunction<L, H>,
    > UpdatedSkeletonTree<L, H, TH> for UpdatedSkeletonTreeImpl<L, H, TH>
{
    fn compute_filled_tree(&self) -> Result<impl FilledTree<L>, SkeletonTreeError> {
        // TODO: Implement this method
        // 1. Create a new hashmap for the filled tree.
        // TODO: add mut
        #[allow(unused_mut)]
        let mut filled_tree_map = Arc::new(RwLock::new(HashMap::new()));
        // TODO: delete this block
        // let tmp_node = FilledNode {
        //     data: NodeData::Binary(BinaryData {
        //         left_hash: HashOutput::ZERO,
        //         right_hash: HashOutput::ZERO,
        //     }),
        //     hash: (HashOutput::ZERO),
        // };
        // let mut write_locked_map = filled_tree_map.write().expect("RwLock poisoned");
        // write_locked_map.insert(HashOutput(Felt::default()), Box::new(tmp_node));

        // 2. Compute the filled tree hashmap from the skeleton_tree.
        let skeleton_tree = self.get_sk_tree();
        let index = NodeIndex::root_index();
        self.compute_filled_tree(skeleton_tree, index, Arc::clone(&filled_tree_map));
        // assert_eq!(_root, _skeleton_tree.get(&NodeIndex(Felt::ONE)).unwrap());
        // 3. Create a new FilledTreeImpl from the hashmap.
        let filled_tree: filled_tree::FilledTreeImpl<L> =
            FilledTreeImpl::new(Arc::clone(&filled_tree_map));
        Ok(filled_tree)
    }
}

// pub fn algorithm_hash(map: &HashMap<Felt, SNTreeNode>, index: StarknetFelt) -> StarknetFelt {
//     let node = map.get(&index).unwrap();
//     if node.is_leaf {
//         return node.hash_value.unwrap();
//     }
//     if !node.is_path {
//         let left_index = index * TWO;
//         let left_value = algorithm_hash(map, left_index);
//         let right_value = algorithm_hash(map, left_index + 1);
//         hash::Pedersen::hash(&left_value, &right_value)
//     } else {
//         todo!("Path hash computation")
//     }
// }

const ONE: Felt = Felt::ONE;
const TWO: Felt = Felt::TWO;
// pub fn compute_filled_tree<L: LeafDataTrait>(
//     map: &HashMap<Felt, SkeletonNode<L>>,
//     index: Felt,
//     output_map: Arc<RwLock<HashMap<Felt, Mutex<Felt>>>>,
// ) -> Felt {
//     let node = &mut map.get(&index).unwrap();
//     let node_hash = match node {
//         SkeletonNode::Binary => todo!(),
//         SkeletonNode::Edge { path_to_bottom } => todo!(),
//         SkeletonNode::Sibling(hash_result) => {
//             let mut write_locked_map = output_map.write().expect("RwLock poisoned");
//             write_locked_map.insert(index, Mutex::new(hash_result.0));
//             return hash_result.0;
//         }
//         SkeletonNode::Leaf(_) => {
//             let mut write_locked_map = output_map.write().expect("RwLock poisoned");
//             write_locked_map.insert(index, Mutex::new(node.hash_value.unwrap()));
//             return node.get_hash().unwrap();
//         }
//         SkeletonNode::Empty => unimplemented!("UpdatedSkeletonTree should not contain Empty nodes"),
//     }
//     // if node.is_leaf {
//     //     let mut write_locked_map = output_map.write().expect("RwLock poisoned");
//     //     write_locked_map.insert(index, Mutex::new(node.hash_value.unwrap()));
//     //     return node.hash_value.unwrap();
//     // }
//     let mut left_value: Felt = Default::default();
//     let mut right_value: Felt = Default::default();
//     let left_index = index * TWO;
//     let right_index = left_index + ONE;
//     // rayon::scope(|s: &Scope<'_>| {
//     //     s.spawn(|_s| {
//     left_value = compute_filled_tree(map, left_index, Arc::clone(&output_map));
//     // });
//     // s.spawn(|_s| {
//     right_value = compute_filled_tree(map, right_index, Arc::clone(&output_map));
//     //     });
//     // });
//     if !node.is_path {
//         let hash_value = hash::Pedersen::hash(&left_value, &right_value);
//         //let read_locked_map = output_map.read().expect("RwLock poisoned");
//         let mut write_locked_map = output_map.write().expect("RwLock poisoned");
//         write_locked_map.insert(index, Mutex::new(hash_value));
//         // drop(read_locked_map);
//         hash_value
//     } else {
//         todo!("Path hash computation")
//     }
// }

// pub fn algorithm_hash_rayon_join(
//     map: &HashMap<StarknetFelt, SNTreeNode>,
//     index: StarknetFelt,
//     output_map: Arc<RwLock<HashMap<StarknetFelt, Mutex<StarknetFelt>>>>,
// ) -> StarknetFelt {
//     let node = &mut map.get(&index).unwrap();
//     if node.is_leaf {
//         return node.hash_value.unwrap();
//     }
//     let mut left_value: StarknetFelt = Default::default();
//     let mut right_value: StarknetFelt = Default::default();
//     let left_index = index * TWO;
//     let right_index = left_index + ONE;
//     rayon::join(
//         || {
//             left_value = algorithm_hash_rayon(map, left_index, Arc::clone(&output_map));
//         },
//         || {
//             right_value = algorithm_hash_rayon(map, right_index, Arc::clone(&output_map));
//         },
//     );
//     if !node.is_path {
//         let hash_value = hash::Pedersen::hash(&left_value, &right_value);
//         //let read_locked_map = output_map.read().expect("RwLock poisoned");
//         let mut write_locked_map = output_map.write().expect("RwLock poisoned");
//         write_locked_map.insert(index, Mutex::new(hash_value));
//         // drop(read_locked_map);
//         hash_value
//     } else {
//         todo!("Path hash computation")
//     }
// }
