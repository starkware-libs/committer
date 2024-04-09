use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use super::filled_node::{EdgeData, NodeData};
use crate::hash::types::HashInputPair;
use crate::hash::types::{HashFunction, HashOutput};
use crate::patricia_merkle_tree::errors::SkeletonTreeError;
use crate::patricia_merkle_tree::filled_node::{BinaryData, FilledNode};
use crate::patricia_merkle_tree::filled_tree::{self, FilledTree, FilledTreeImpl};
use crate::patricia_merkle_tree::skeleton_node::{compute_bottom_index, SkeletonNode};
use crate::patricia_merkle_tree::types::{LeafDataTrait, NodeIndex, TreeHashFunction};
use crate::types::{ONE, TWO};

/// Consider a Patricia-Merkle Tree which should be updated with new leaves.
/// This trait represents the structure of the subtree which will be modified in the
/// update. It also contains the hashes of the Sibling nodes on the Merkle paths from the
/// updated leaves to the root.

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

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// This trait represents the structure of the subtree which was modified in the update.
/// It also contains the hashes of the Sibling nodes on the Merkle paths from the updated leaves
/// to the root.
pub(crate) trait UpdatedSkeletonTree<L: LeafDataTrait, H: HashFunction, TH: TreeHashFunction<L, H>>
{
    /// Computes and returns the filled tree.
    fn compute_filled_tree(&self) -> Result<impl FilledTree<L>, SkeletonTreeError>;
}

#[allow(dead_code)]
struct UpdatedSkeletonTreeImpl<
    L: LeafDataTrait + std::clone::Clone + std::marker::Sync,
    H: HashFunction + std::marker::Sync,
    TH: TreeHashFunction<L, H> + std::marker::Sync,
> {
    skeleton_tree: HashMap<NodeIndex, SkeletonNode<L>>,
    hash_function: H,
    tree_hash_function: TH,
}

#[allow(dead_code)]
impl<
        L: LeafDataTrait + std::clone::Clone + std::marker::Sync + std::marker::Send,
        H: HashFunction + std::marker::Sync,
        TH: TreeHashFunction<L, H> + std::marker::Sync,
    > UpdatedSkeletonTreeImpl<L, H, TH>
{
    fn get_sk_tree(&self) -> &HashMap<NodeIndex, SkeletonNode<L>> {
        &self.skeleton_tree
    }
    fn get_node(&self, index: NodeIndex) -> Option<&SkeletonNode<L>> {
        self.skeleton_tree.get(&index)
    }

    pub fn compute_filled_tree(
        map: &HashMap<NodeIndex, SkeletonNode<L>>,
        index: NodeIndex,
        output_map: Arc<RwLock<HashMap<NodeIndex, Mutex<FilledNode<L>>>>>,
    ) -> HashOutput {
        let node = &mut map.get(&index).unwrap();
        let node_hash = match node {
            SkeletonNode::Binary => {
                let left_index = NodeIndex(index.0 * TWO);
                let right_index = NodeIndex(left_index.0 + ONE);

                let mut left_hash = Default::default();
                let mut right_hash = Default::default();

                rayon::join(
                    || {
                        left_hash =
                            Self::compute_filled_tree(map, left_index, Arc::clone(&output_map));
                    },
                    || {
                        right_hash =
                            Self::compute_filled_tree(map, right_index, Arc::clone(&output_map));
                    },
                );

                let hash_value = H::compute_hash(HashInputPair(left_hash.0, right_hash.0));
                let mut write_locked_map = output_map.write().expect("RwLock poisoned");
                write_locked_map.insert(
                    index,
                    Mutex::new(FilledNode {
                        hash: hash_value.clone(),
                        data: NodeData::Binary(BinaryData {
                            left_hash,
                            right_hash,
                        }),
                    }),
                );
                hash_value
            }
            SkeletonNode::Edge { path_to_bottom } => {
                let bottom_node_index = compute_bottom_index(index, path_to_bottom.clone());
                let bottom_node_hash =
                    Self::compute_filled_tree(map, bottom_node_index, Arc::clone(&output_map));
                let hash_value =
                    H::compute_hash(HashInputPair(bottom_node_hash.0, path_to_bottom.path.0))
                        + path_to_bottom.length.clone();
                let mut write_locked_map = output_map.write().expect("RwLock poisoned");
                write_locked_map.insert(
                    index,
                    Mutex::new(FilledNode {
                        hash: hash_value.clone(),
                        data: NodeData::Edge(EdgeData {
                            path_to_bottom: path_to_bottom.clone(),
                            bottom_hash: bottom_node_hash,
                        }),
                    }),
                );
                hash_value
            }
            SkeletonNode::Sibling(hash_result) => hash_result.clone(),
            SkeletonNode::Leaf(node_data) => {
                let mut write_locked_map = output_map.write().expect("RwLock poisoned");
                let hash_value = TH::compute_node_hash(NodeData::Leaf(node_data.clone()));
                write_locked_map.insert(
                    index,
                    Mutex::new(FilledNode {
                        hash: hash_value.clone(),
                        data: NodeData::Leaf(node_data.clone()),
                    }),
                );
                return hash_value;
            }
            SkeletonNode::Empty => {
                unimplemented!("UpdatedSkeletonTree should not contain Empty nodes!")
            }
        };
        node_hash
    }
}

impl<
        L: LeafDataTrait + std::fmt::Debug + std::clone::Clone + std::marker::Sync + std::marker::Send,
        H: HashFunction + std::marker::Sync,
        TH: TreeHashFunction<L, H> + std::marker::Sync,
    > UpdatedSkeletonTree<L, H, TH> for UpdatedSkeletonTreeImpl<L, H, TH>
{
    fn compute_filled_tree(&self) -> Result<impl FilledTree<L>, SkeletonTreeError> {
        // 1. Create a new hashmap for the filled tree.
        let filled_tree_map = Arc::new(RwLock::new(HashMap::new()));
        // 2. Compute the filled tree hashmap from the skeleton_tree.
        let skeleton_tree = self.get_sk_tree();
        let index = NodeIndex::root_index();
        Self::compute_filled_tree(skeleton_tree, index, Arc::clone(&filled_tree_map));
        // 3. Create a new FilledTreeImpl from the hashmap.
        let filled_tree: filled_tree::FilledTreeImpl<L> =
            FilledTreeImpl::new(Arc::clone(&filled_tree_map));
        Ok(filled_tree)
    }
}
