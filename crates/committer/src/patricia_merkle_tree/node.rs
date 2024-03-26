use crate::types::CommitterFelt;

use crate::hash::types::{HashFunction, HashOutput};

pub(crate) struct NodeIndex(pub CommitterFelt);

#[allow(dead_code)]
pub(crate) enum NodeType {
    Binary,
    Edge,
    Leaf,
}

#[allow(dead_code)]
pub(crate) enum NodeValue {
    Binary {
        hash: Option<HashOutput>,
    },
    Edge {
        path: EdgePath,
        bottom_value: Option<CommitterFelt>,
    },
    Leaf(CommitterFelt),
}

#[allow(dead_code)]
pub(crate) struct EdgePath {
    pub path: CommitterFelt,
    pub length: CommitterFelt,
}

pub(crate) trait Node<H: HashFunction> {
    /// Returns node's value.
    fn get_value(&self) -> NodeValue;

    /// Computes and sets node's hash if possible (i.e., if node is not a leaf and all non-leaf
    /// children's hashes are set).
    fn compute_and_set_hash(&mut self);

    /// Returns node's type.
    fn get_type(&self) -> NodeType;

    /// Returns parent of node, if it exists.
    fn get_parent(&mut self) -> Option<&mut Self>;

    /// Returns left child of node, if it exists.
    fn get_left_child(&mut self) -> Option<&mut Self>;

    /// Returns right child of node, if it exists.
    fn get_right_child(&mut self) -> Option<&mut Self>;
}
