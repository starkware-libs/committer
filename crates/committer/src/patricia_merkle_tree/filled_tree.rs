use std::collections::HashMap;

use crate::patricia_merkle_tree::filled_node::FilledNode;
use crate::patricia_merkle_tree::types::{LeafTrait, NodeIndex};

pub(crate) trait FilledTree<L: LeafTrait> {
    fn get_all_nodes(&self) -> HashMap<NodeIndex, &FilledNode<L>>;
}
