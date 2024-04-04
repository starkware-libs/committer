use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::hash::types::HashOutput;
use crate::patricia_merkle_tree::filled_node::FilledNode;
use crate::patricia_merkle_tree::types::LeafDataTrait;

pub(crate) trait FilledTree<L: LeafDataTrait> {
    fn get_all_nodes(&self) -> Arc<RwLock<HashMap<HashOutput, Box<FilledNode<L>>>>>;
}

pub(crate) struct FilledTreeImpl<L: LeafDataTrait> {
    tree_map: Arc<RwLock<HashMap<HashOutput, Box<FilledNode<L>>>>>,
}

impl<L: LeafDataTrait> FilledTreeImpl<L> {
    pub(crate) fn new(tree_map: Arc<RwLock<HashMap<HashOutput, Box<FilledNode<L>>>>>) -> Self {
        Self { tree_map }
    }
}

impl<L: LeafDataTrait> FilledTree<L> for FilledTreeImpl<L> {
    fn get_all_nodes(&self) -> Arc<RwLock<HashMap<HashOutput, Box<FilledNode<L>>>>> {
        Arc::clone(&self.tree_map)
    }
}
