
use crate::types::Felt;
use serde::{Serialize, Deserialize};

// Const describe the size of the serialised node.
pub(crate) const SERIALISE_HASH_SIZE: usize = 32;
pub(crate) const BINARY_SIZE: usize = 2*SERIALISE_HASH_SIZE;
pub(crate) const EDGE_SIZE: usize = 65;
pub(crate) const STORAGE_LEAF_SIZE: usize = SERIALISE_HASH_SIZE;


// Enum to describe the serialised node.
#[allow(dead_code)]
pub(crate) enum SerialiseNode{
    Binary([u8; BINARY_SIZE]),
    Edge([u8; EDGE_SIZE]),
    CompiledClassLeaf(Vec<u8>),
    StorageLeaf([u8; STORAGE_LEAF_SIZE]),
    StateTreeLeaf(Vec<u8>),
}

// Temporary struct to serialise the leaf CompiledClass.
#[derive(Serialize, Deserialize)]
pub(crate) struct LeafCompiledClassToSerialise {
    pub(crate) compiled_class_hash: Felt,
}
