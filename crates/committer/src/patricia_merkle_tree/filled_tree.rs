use std::collections::HashMap;

use crate::patricia_merkle_tree::filled_node::FilledNode;
use crate::patricia_merkle_tree::types::{LeafDataTrait, NodeIndex};

pub(crate) trait FilledTree<L: LeafDataTrait> {
    fn get_all_nodes(&self) -> &HashMap<NodeIndex, Box<FilledNode<L>>>;
}

pub(crate) struct FilledTreeImpl<L: LeafDataTrait> {
    tree_map: HashMap<NodeIndex, Box<FilledNode<L>>>,
}

impl<L: LeafDataTrait> FilledTreeImpl<L> {
    pub(crate) fn new(tree_map: HashMap<NodeIndex, Box<FilledNode<L>>>) -> Self {
        Self { tree_map }
    }
}

impl<L: LeafDataTrait> FilledTree<L> for FilledTreeImpl<L> {
    fn get_all_nodes(&self) -> &HashMap<NodeIndex, Box<FilledNode<L>>> {
        &self.tree_map
    }
}
