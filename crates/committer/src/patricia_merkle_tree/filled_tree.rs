use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::patricia_merkle_tree::filled_node::FilledNode;
use crate::patricia_merkle_tree::types::LeafDataTrait;

use super::types::NodeIndex;

/// Consider a Patricia-Merkle Tree which has been updated with new leaves.
/// FilledTree consists of all nodes which were modified in the update, including their updated
/// data and hashes.
pub(crate) trait FilledTree<L: LeafDataTrait> {
    fn get_all_nodes(&self) -> Arc<RwLock<HashMap<NodeIndex, Mutex<FilledNode<L>>>>>;
}

pub(crate) struct FilledTreeImpl<L: LeafDataTrait> {
    tree_map: Arc<RwLock<HashMap<NodeIndex, Mutex<FilledNode<L>>>>>,
}

impl<L: LeafDataTrait> FilledTreeImpl<L> {
    pub(crate) fn new(tree_map: Arc<RwLock<HashMap<NodeIndex, Mutex<FilledNode<L>>>>>) -> Self {
        Self { tree_map }
    }
}

impl<L: LeafDataTrait> FilledTree<L> for FilledTreeImpl<L> {
    fn get_all_nodes(&self) -> Arc<RwLock<HashMap<NodeIndex, Mutex<FilledNode<L>>>>> {
        Arc::clone(&self.tree_map)
    }
}
