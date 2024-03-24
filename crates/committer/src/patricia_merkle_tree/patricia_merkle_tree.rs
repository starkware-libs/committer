use crate::patricia_merkle_tree::patricia_merkle_node::PatriciaMerkleNode;

pub struct StarkFelt([u8; 32]);

trait PatriciaMerkleTree {
    /// Returns the node with given full (Merkle) index, if it exists.
    fn get_node(full_index: u128) -> Option<PatriciaMerkleNode>;

    /// Returns the root if the tree is not empty.
    fn get_root() -> Option<PatriciaMerkleNode>;

    /// Computes the hash of the given node.
    fn compute_hash(node: PatriciaMerkleNode) -> StarkFelt;
}
