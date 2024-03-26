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
    Binary { hash: Option<HashOutput> },
    Edge { bottom_value: Option<CommitterFelt> },
    Leaf(LeafValue),
}

#[allow(dead_code)]
pub(crate) enum LeafValue {
    StorageKey(CommitterFelt),
    CompiledClassHash(CommitterFelt),
    StateTreeValue {
        class_hash: CommitterFelt,
        contract_state_root_hash: CommitterFelt,
        nonce: CommitterFelt,
    },
}

#[allow(dead_code)]
pub(crate) struct EdgePath {
    pub path: CommitterFelt,
    pub length: CommitterFelt,
}

pub(crate) trait Node<H: HashFunction> {
    /// Returns node's value, if it is set.
    fn get_value(&self) -> NodeValue;

    /// If binary - computes and sets hash if children's hashes are set.
    /// If edge - sets value to bottom's value, if bottom's value is set.
    /// If leaf - does nothing, as value is already set.
    fn set_value(&mut self);

    /// If edge - return path to bottom node.
    fn get_path(&self) -> Option<EdgePath>;

    /// Returns node's type.
    fn get_type(&self) -> NodeType;

    /// Returns parent of node, if it exists.
    fn get_parent(&mut self) -> Option<&mut Self>;

    /// Returns left child of node, if it exists.
    fn get_left_child(&mut self) -> Option<&mut Self>;

    /// Returns right child of node, if it exists.
    fn get_right_child(&mut self) -> Option<&mut Self>;
}
