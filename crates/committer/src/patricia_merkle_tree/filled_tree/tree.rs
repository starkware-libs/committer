use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use async_recursion::async_recursion;

use crate::hash::hash_trait::HashFunction;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::node::FilledNode;
use crate::patricia_merkle_tree::node_data::inner_node::BinaryData;
use crate::patricia_merkle_tree::node_data::inner_node::EdgeData;
use crate::patricia_merkle_tree::node_data::inner_node::NodeData;
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use crate::patricia_merkle_tree::node_data::leaf::LeafModifications;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::updated_skeleton_tree::hash_function::TreeHashFunction;
use crate::patricia_merkle_tree::updated_skeleton_tree::node::UpdatedSkeletonNode;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTree;
use crate::storage::db_object::DBObject;
use crate::storage::storage_trait::StorageKey;
use crate::storage::storage_trait::StorageValue;

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// FilledTree consists of all nodes which were modified in the update, including their updated
/// data and hashes.
pub(crate) trait FilledTree<L: LeafData>: Sized {
    /// Computes and returns the filled tree.
    #[allow(dead_code)]
    async fn create<H: HashFunction, TH: TreeHashFunction<L, H>>(
        updated_skeleton: impl UpdatedSkeletonTree,
        leaf_modifications: &LeafModifications<L>,
    ) -> Result<Self, FilledTreeError<L>>;

    /// Serializes the current state of the tree into a hashmap,
    /// where each key-value pair corresponds
    /// to a storage key and its serialized storage value.
    #[allow(dead_code)]
    fn serialize(&self) -> HashMap<StorageKey, StorageValue>;

    #[allow(dead_code)]
    fn get_root_hash(&self) -> Result<HashOutput, FilledTreeError<L>>;
}

pub(crate) struct FilledTreeImpl {
    tree_map: HashMap<NodeIndex, FilledNode<LeafDataImpl>>,
}

impl FilledTreeImpl {
    fn initialize_with_placeholders(
        updated_skeleton: &impl UpdatedSkeletonTree,
    ) -> HashMap<NodeIndex, Mutex<Option<FilledNode<LeafDataImpl>>>> {
        let mut filled_tree_map = HashMap::new();
        for (index, node) in updated_skeleton.get_nodes() {
            if !matches!(node, UpdatedSkeletonNode::Sibling(_)) {
                filled_tree_map.insert(index, Mutex::new(None));
            }
        }
        filled_tree_map
    }

    #[allow(dead_code)]
    pub(crate) fn get_all_nodes(&self) -> &HashMap<NodeIndex, FilledNode<LeafDataImpl>> {
        &self.tree_map
    }

    /// Writes the hash and data to the output map. The writing is done in a thread-safe manner with
    /// interior mutability to avoid thread contention.
    fn write_to_output_map(
        output_map: Arc<HashMap<NodeIndex, Mutex<Option<FilledNode<LeafDataImpl>>>>>,
        index: NodeIndex,
        hash: HashOutput,
        data: NodeData<LeafDataImpl>,
    ) -> Result<(), FilledTreeError<LeafDataImpl>> {
        match output_map.get(&index) {
            Some(node) => {
                let mut node = node
                    .lock()
                    .map_err(|_| FilledTreeError::PoisonedLock("Cannot lock node.".to_owned()))?;
                match node.take() {
                    Some(existing_node) => Err(FilledTreeError::<LeafDataImpl>::DoubleUpdate {
                        index,
                        existing_value: Box::new(existing_node),
                    }),
                    None => {
                        *node = Some(FilledNode { hash, data });
                        Ok(())
                    }
                }
            }
            None => Err(FilledTreeError::<LeafDataImpl>::MissingNode(index)),
        }
    }

    fn remove_arc_mutex_and_option(
        hash_map_in: Arc<HashMap<NodeIndex, Mutex<Option<FilledNode<LeafDataImpl>>>>>,
    ) -> Result<HashMap<NodeIndex, FilledNode<LeafDataImpl>>, FilledTreeError<LeafDataImpl>> {
        let mut hash_map_out = HashMap::new();
        for (key, value) in hash_map_in.iter() {
            let mut value = value.lock().map_err(|_| {
                FilledTreeError::<LeafDataImpl>::PoisonedLock("Cannot lock node.".to_owned())
            })?;
            match value.take() {
                Some(value) => {
                    hash_map_out.insert(*key, value);
                }
                None => return Err(FilledTreeError::<LeafDataImpl>::MissingNode(*key)),
            }
        }
        Ok(hash_map_out)
    }

    #[async_recursion]
    async fn compute_filled_tree_rec<H, TH>(
        updated_skeleton: &impl UpdatedSkeletonTree,
        index: NodeIndex,
        leaf_modifications: &LeafModifications<LeafDataImpl>,
        output_map: Arc<HashMap<NodeIndex, Mutex<Option<FilledNode<LeafDataImpl>>>>>,
    ) -> Result<HashOutput, FilledTreeError<LeafDataImpl>>
    where
        H: HashFunction,
        TH: TreeHashFunction<LeafDataImpl, H>,
    {
        let node = updated_skeleton.get_node(index)?;
        match node {
            UpdatedSkeletonNode::Binary => {
                let left_index = index * 2.into();
                let right_index = left_index + NodeIndex::ROOT;

                let (left_hash, right_hash) = tokio::join!(
                    Self::compute_filled_tree_rec::<H, TH>(
                        updated_skeleton,
                        left_index,
                        leaf_modifications,
                        Arc::clone(&output_map)
                    ),
                    Self::compute_filled_tree_rec::<H, TH>(
                        updated_skeleton,
                        right_index,
                        leaf_modifications,
                        Arc::clone(&output_map)
                    ),
                );

                let data = NodeData::Binary(BinaryData {
                    left_hash: left_hash?,
                    right_hash: right_hash?,
                });

                let hash_value = TH::compute_node_hash(&data);
                Self::write_to_output_map(output_map, index, hash_value, data)?;
                Ok(hash_value)
            }
            UpdatedSkeletonNode::Edge { path_to_bottom } => {
                let bottom_node_index = NodeIndex::compute_bottom_index(index, path_to_bottom);
                let bottom_hash = Self::compute_filled_tree_rec::<H, TH>(
                    updated_skeleton,
                    bottom_node_index,
                    leaf_modifications,
                    Arc::clone(&output_map),
                )
                .await?;
                let data = NodeData::Edge(EdgeData {
                    path_to_bottom: *path_to_bottom,
                    bottom_hash,
                });
                let hash_value = TH::compute_node_hash(&data);
                Self::write_to_output_map(output_map, index, hash_value, data)?;
                Ok(hash_value)
            }
            UpdatedSkeletonNode::Sibling(hash_result) => Ok(*hash_result),
            UpdatedSkeletonNode::Leaf(skeleton_leaf) => {
                let leaf_data = leaf_modifications
                    .get(&index)
                    .ok_or(FilledTreeError::<LeafDataImpl>::MissingDataForUpdate(index))?
                    .clone();
                if skeleton_leaf.is_zero() != leaf_data.is_empty() {
                    return Err(FilledTreeError::<LeafDataImpl>::InconsistentModification {
                        index,
                        skeleton_leaf: *skeleton_leaf,
                    });
                }
                let node_data = NodeData::Leaf(leaf_data);
                let hash_value = TH::compute_node_hash(&node_data);
                Self::write_to_output_map(output_map, index, hash_value, node_data)?;
                Ok(hash_value)
            }
        }
    }
}

impl FilledTree<LeafDataImpl> for FilledTreeImpl {
    async fn create<H: HashFunction, TH: TreeHashFunction<LeafDataImpl, H>>(
        updated_skeleton: impl UpdatedSkeletonTree,
        leaf_modifications: &LeafModifications<LeafDataImpl>,
    ) -> Result<Self, FilledTreeError<LeafDataImpl>> {
        // Compute the filled tree in two steps:
        //   1. Create a map containing the tree structure without hash values.
        //   2. Fill in the hash values.
        let filled_tree_map = Arc::new(Self::initialize_with_placeholders(&updated_skeleton));

        Self::compute_filled_tree_rec::<H, TH>(
            &updated_skeleton,
            NodeIndex::ROOT,
            leaf_modifications,
            Arc::clone(&filled_tree_map),
        )
        .await?;

        // Create and return a new FilledTreeImpl from the hashmap.
        Ok(FilledTreeImpl {
            tree_map: Self::remove_arc_mutex_and_option(filled_tree_map)?,
        })
    }

    fn serialize(&self) -> HashMap<StorageKey, StorageValue> {
        // This function iterates over each node in the tree, using the node's `db_key` as the hashmap key
        // and the result of the node's `serialize` method as the value.
        self.get_all_nodes()
            .iter()
            .map(|(_, node)| (node.db_key(), node.serialize()))
            .collect()
    }

    fn get_root_hash(&self) -> Result<HashOutput, FilledTreeError<LeafDataImpl>> {
        match self.tree_map.get(&NodeIndex::ROOT) {
            Some(root_node) => Ok(root_node.hash),
            None => Err(FilledTreeError::MissingRoot),
        }
    }
}
