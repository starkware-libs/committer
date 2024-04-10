use crate::types::Felt;
use serde::{Deserialize, Serialize};

// Const describe the size of the serialized node.
pub(crate) const SERIALIZE_HASH_BYTES: usize = 32;
pub(crate) const BINARY_BYTES: usize = 2 * SERIALIZE_HASH_BYTES;
pub(crate) const EDGE_LENGTH_BYTES: usize = 1;
pub(crate) const EDGE_PATH_BYTES: usize = 32;
pub(crate) const EDGE_BYTES: usize = SERIALIZE_HASH_BYTES + EDGE_PATH_BYTES + EDGE_LENGTH_BYTES;
pub(crate) const STORAGE_LEAF_SIZE: usize = SERIALIZE_HASH_BYTES;

/// Enum to describe the serialized node.
#[allow(dead_code)]
pub(crate) enum SerializeNode {
    Binary([u8; BINARY_BYTES]),
    Edge([u8; EDGE_BYTES]),
    CompiledClassLeaf(Vec<u8>),
    StorageLeaf([u8; STORAGE_LEAF_SIZE]),
    StateTreeLeaf(Vec<u8>),
}

/// Temporary struct to serialize the leaf CompiledClass.
/// Required to comply to existing storage layout.
#[derive(Serialize, Deserialize)]
pub(crate) struct LeafCompiledClassToSerialize {
    pub(crate) compiled_class_hash: Felt,
}
