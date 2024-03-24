use std::iter::Map;

use crate::patricia_merkle_tree::filled_node::FilledNode;
use crate::patricia_merkle_tree::types::{Leaf, NodeIndex};

pub(crate) trait FilledTree<L: Leaf> {
    fn get_all_nodes(&self) -> Map<NodeIndex, &FilledNode<L>>;
}
