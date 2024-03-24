use crate::patricia_merkle_tree::node::Node;

use super::{hash::HashFunction, node::NodeIndex};

pub(crate) trait Tree<H: HashFunction, N: Node<H>> {
    /// Returns the node with given full (Merkle) index, if it exists.
    fn get_node(full_index: NodeIndex) -> Option<N>;

    /// Returns the root if the tree is not empty.
    fn get_root() -> Option<N>;
}
