use crate::patricia_merkle_tree::node::Node;


trait Tree {
    /// Returns the node with given full (Merkle) index, if it exists.
    fn get_node(full_index: u128) -> Option<Node>;

    /// Returns the root if the tree is not empty.
    fn get_root() -> Option<Node>;

    /// Computes the hash of the given node.
    fn compute_hash(node: Node) -> [u8; 32];
}
