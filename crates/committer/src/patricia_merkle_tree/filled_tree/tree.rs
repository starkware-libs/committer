use std::collections::HashMap;
use std::collections::HashSet;
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
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTree;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::updated_skeleton_tree::errors::UpdatedSkeletonTreeError;
use crate::patricia_merkle_tree::updated_skeleton_tree::hash_function::TreeHashFunction;
use crate::patricia_merkle_tree::updated_skeleton_tree::node::UpdatedSkeletonNode;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTree;
use crate::storage::storage_trait::Storage;
use crate::storage::storage_trait::StorageKey;

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// FilledTree consists of all nodes which were modified in the update, including their updated
/// data and hashes.
pub(crate) trait FilledTree<
    L: LeafData + std::clone::Clone + Sync + Send,
    O: OriginalSkeletonTree<L>,
    U: UpdatedSkeletonTree<L, O>,
>: Sized
{
    /// Computes and returns the filled tree.
    #[allow(dead_code)]
    async fn create<H: HashFunction, TH: TreeHashFunction<L, H>>(
        updated_tree: U,
    ) -> Result<Self, UpdatedSkeletonTreeError<L>>;

    /// Serializes the tree into storage. Returns hash set of keys of the serialized nodes,
    /// if successful.
    #[allow(dead_code)]
    fn serialize(&self, storage: &mut impl Storage)
        -> Result<HashSet<StorageKey>, FilledTreeError>;

    #[allow(dead_code)]
    fn get_root_hash(&self) -> Result<HashOutput, FilledTreeError>;
}

pub(crate) struct FilledTreeImpl<
    L: LeafData + std::clone::Clone + Sync + Send,
    O: OriginalSkeletonTree<L>,
    U: UpdatedSkeletonTree<L, O>,
> {
    tree_map: HashMap<NodeIndex, FilledNode<L>>,
    phantom_o: std::marker::PhantomData<O>,
    phantom_u: std::marker::PhantomData<U>,
}

impl<
        L: LeafData + std::clone::Clone + Sync + Send,
        O: OriginalSkeletonTree<L>,
        U: UpdatedSkeletonTree<L, O> + Sync + Send,
    > FilledTreeImpl<L, O, U>
{
    #[allow(dead_code)]
    pub(crate) fn new(tree_map: HashMap<NodeIndex, FilledNode<L>>) -> Self {
        Self {
            tree_map,
            phantom_o: std::marker::PhantomData,
            phantom_u: std::marker::PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_all_nodes(&self) -> &HashMap<NodeIndex, FilledNode<L>> {
        &self.tree_map
    }

    /// Writes the hash and data to the output map. The writing is done in a thread-safe manner with
    /// interior mutability to avoid thread contention.
    fn write_to_output_map(
        output_map: Arc<HashMap<NodeIndex, Mutex<Option<FilledNode<L>>>>>,
        index: NodeIndex,
        hash: HashOutput,
        data: NodeData<L>,
    ) -> Result<(), UpdatedSkeletonTreeError<L>> {
        match output_map.get(&index) {
            Some(node) => {
                let mut node = node.lock().map_err(|_| {
                    UpdatedSkeletonTreeError::PoisonedLock("Cannot lock node.".to_owned())
                })?;
                match node.take() {
                    Some(existing_node) => Err(UpdatedSkeletonTreeError::DoubleUpdate {
                        index,
                        existing_value: Box::new(existing_node),
                    }),
                    None => {
                        *node = Some(FilledNode { hash, data });
                        Ok(())
                    }
                }
            }
            None => Err(UpdatedSkeletonTreeError::MissingNode(index)),
        }
    }

    fn remove_arc_mutex_and_option(
        hash_map_in: Arc<HashMap<NodeIndex, Mutex<Option<FilledNode<L>>>>>,
    ) -> Result<HashMap<NodeIndex, FilledNode<L>>, UpdatedSkeletonTreeError<L>>
    where
        L: Send + Sync,
    {
        let mut hash_map_out = HashMap::new();
        for (key, value) in hash_map_in.iter() {
            let mut value = value.lock().map_err(|_| {
                UpdatedSkeletonTreeError::PoisonedLock("Cannot lock node.".to_owned())
            })?;
            match value.take() {
                Some(value) => {
                    hash_map_out.insert(*key, value);
                }
                None => return Err(UpdatedSkeletonTreeError::MissingNode(*key)),
            }
        }
        Ok(hash_map_out)
    }

    #[async_recursion]
    async fn compute_filled_tree_rec<H, TH>(
        updated_tree: &U,
        index: NodeIndex,
        output_map: Arc<HashMap<NodeIndex, Mutex<Option<FilledNode<L>>>>>,
    ) -> Result<HashOutput, UpdatedSkeletonTreeError<L>>
    where
        L: Sync + Send + 'async_recursion,
        H: HashFunction,
        TH: TreeHashFunction<L, H>,
    {
        let node = updated_tree.get_node(index)?;
        match node {
            UpdatedSkeletonNode::Binary => {
                let left_index = index * 2.into();
                let right_index = left_index + NodeIndex::ROOT;

                let (left_hash, right_hash) = tokio::join!(
                    Self::compute_filled_tree_rec::<H, TH>(
                        updated_tree,
                        left_index,
                        Arc::clone(&output_map)
                    ),
                    Self::compute_filled_tree_rec::<H, TH>(
                        updated_tree,
                        right_index,
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
                    updated_tree,
                    bottom_node_index,
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
            UpdatedSkeletonNode::Leaf(node_data) => {
                let data = NodeData::Leaf(node_data.clone());
                let hash_value = TH::compute_node_hash(&data);
                Self::write_to_output_map(output_map, index, hash_value, data)?;
                Ok(hash_value)
            }
        }
    }
}

impl<
        L: LeafData + std::clone::Clone + Sync + Send,
        O: OriginalSkeletonTree<L>,
        U: UpdatedSkeletonTree<L, O> + Sync + Send,
    > FilledTree<L, O, U> for FilledTreeImpl<L, O, U>
{
    async fn create<H: HashFunction, TH: TreeHashFunction<L, H>>(
        updated_tree: U,
    ) -> Result<Self, UpdatedSkeletonTreeError<L>> {
        // Compute the filled tree in two steps:
        //   1. Create a map containing the tree structure without hash values.
        //   2. Fill in the hash values.
        let mut filled_tree_map = HashMap::new();
        for (index, node) in updated_tree.get_nodes() {
            if !matches!(node, UpdatedSkeletonNode::Sibling(_)) {
                filled_tree_map.insert(*index, Mutex::new(None));
            }
        }
        let filled_tree_map = Arc::new(filled_tree_map);

        Self::compute_filled_tree_rec::<H, TH>(
            &updated_tree,
            NodeIndex::ROOT,
            Arc::clone(&filled_tree_map),
        )
        .await?;

        // Create and return a new FilledTreeImpl from the hashmap.
        Ok(FilledTreeImpl::new(Self::remove_arc_mutex_and_option(
            filled_tree_map,
        )?))
    }

    fn serialize(
        &self,
        _storage: &mut impl Storage,
    ) -> Result<HashSet<StorageKey>, FilledTreeError> {
        todo!()
    }
    fn get_root_hash(&self) -> Result<HashOutput, FilledTreeError> {
        match self.tree_map.get(&NodeIndex::ROOT) {
            Some(root_node) => Ok(root_node.hash),
            None => Err(FilledTreeError::MissingRoot),
        }
    }
}
