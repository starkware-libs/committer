use std::collections::HashMap;

use super::filled_node::NodeData;
#[allow(unused_imports)]
use crate::hash::types::{HashFunction, HashOutput};
use crate::patricia_merkle_tree::errors::SkeletonTreeError;
#[allow(unused_imports)]
use crate::patricia_merkle_tree::filled_node::{BinaryData, FilledNode};
use crate::patricia_merkle_tree::filled_tree::{self, FilledTree, FilledTreeImpl};
use crate::patricia_merkle_tree::skeleton_node::SkeletonNode;
use crate::patricia_merkle_tree::types::{LeafDataTrait, NodeIndex, TreeHashFunction};

/// Consider a Patricia-Merkle Tree which should be updated with new leaves.
/// This trait represents the structure of the subtree which will be modified in the
/// update. It also contains the hashes of the Sibling nodes on the Merkle paths from the
/// updated leaves to the root.

pub(crate) trait CurrentSkeletonTree<L: LeafDataTrait, H: HashFunction, TH: TreeHashFunction<L, H>>
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
    // TODO: add get_root impl
    // TODO: add get_preimage impl
    fn compute_filled_tree(&self) -> Result<impl FilledTree<L>, SkeletonTreeError>;
}

#[allow(dead_code)]
struct UpdatedSkeletonTreeImpl<L: LeafDataTrait> {
    root: SkeletonNode<L>,
    skeleton_tree: HashMap<NodeIndex, SkeletonNode<L>>,
}

#[allow(dead_code)]
impl<L: LeafDataTrait> UpdatedSkeletonTreeImpl<L> {
    fn get_root(&self) -> &SkeletonNode<L> {
        &self.root
    }
    fn get_sk_tree(&self) -> &HashMap<NodeIndex, SkeletonNode<L>> {
        &self.skeleton_tree
    }
    fn get_node(&self, index: NodeIndex) -> Option<&SkeletonNode<L>> {
        self.skeleton_tree.get(&index)
    }
}

impl<L: LeafDataTrait, H: HashFunction, TH: TreeHashFunction<L, H>> UpdatedSkeletonTree<L, H, TH>
    for UpdatedSkeletonTreeImpl<L>
{
    fn compute_filled_tree(&self) -> Result<impl FilledTree<L>, SkeletonTreeError> {
        // TODO: Implement this method
        // 1. Create a new hashmap for the filled tree.
        // TODO: add mut
        #[allow(unused_mut)]
        let mut filled_tree_map = HashMap::new();
        // TODO: delete this block
        let tmp_node = FilledNode {
            data: NodeData::Binary(BinaryData {
                left_hash: HashOutput::ZERO,
                right_hash: HashOutput::ZERO,
            }),
            hash: (HashOutput::ZERO),
        };
        filled_tree_map.insert(NodeIndex::root_index(), Box::new(tmp_node));
        // 2. Compute the filled tree hashmap from the skeleton_tree.
        let _skeleton_tree = self.get_sk_tree();
        let _index = NodeIndex::root_index();
        //compute_hash(skeleton_tree, index, filled_tree_map);
        // assert_eq!(_root, _skeleton_tree.get(&NodeIndex(Felt::ONE)).unwrap());
        // 3. Create a new FilledTreeImpl from the hashmap.
        let filled_tree: filled_tree::FilledTreeImpl<L> = FilledTreeImpl::new(filled_tree_map);
        Ok(filled_tree)
    }
}
