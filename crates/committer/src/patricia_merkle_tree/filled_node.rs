use crate::patricia_merkle_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::FilledTreeResult;
use crate::patricia_merkle_tree::serialised_node::{
    LeafCompiledClassToSerialise, SerialiseNode, BINARY_BYTES, EDGE_BYTES, SERIALISE_HASH_BYTES,
};
use crate::patricia_merkle_tree::types::{EdgeData, LeafDataTrait};
use crate::{hash::types::HashOutput, types::Felt};

// TODO(Nimrod, 1/6/2024): Swap to starknet-types-core types once implemented.
#[allow(dead_code)]
pub(crate) struct ClassHash(pub Felt);
#[allow(dead_code)]
pub(crate) struct Nonce(pub Felt);

#[allow(dead_code)]
/// A node in a Patricia-Merkle tree which was modified during an update.
pub(crate) struct FilledNode<L: LeafDataTrait> {
    pub(crate) hash: HashOutput,
    data: NodeData<L>,
}

#[allow(dead_code)]
// A Patricia-Merkle tree node's data, i.e., the pre-image of its hash.
pub(crate) enum NodeData<L: LeafDataTrait> {
    Binary(BinaryData),
    Edge(EdgeData),
    Leaf(L),
}

#[allow(dead_code)]
pub(crate) struct BinaryData {
    left_hash: HashOutput,
    right_hash: HashOutput,
}

#[allow(dead_code)]
pub(crate) enum LeafData {
    StorageValue(Felt),
    CompiledClassHash(ClassHash),
    StateTreeTuple {
        class_hash: ClassHash,
        contract_state_root_hash: Felt,
        nonce: Nonce,
    },
}

impl LeafDataTrait for LeafData {
    fn is_empty(&self) -> bool {
        match self {
            LeafData::StorageValue(value) => *value == Felt::ZERO,
            LeafData::CompiledClassHash(class_hash) => class_hash.0 == Felt::ZERO,
            LeafData::StateTreeTuple {
                class_hash,
                contract_state_root_hash,
                nonce,
            } => {
                nonce.0 == Felt::ZERO
                    && class_hash.0 == Felt::ZERO
                    && *contract_state_root_hash == Felt::ZERO
            }
        }
    }
}

impl LeafData {
    /// Serialises the leaf data into a JSON string.
    /// For storage values, string which represents the value.
    /// For compiled class hashes or state tree tuples,
    /// using temp structs which define in serialised_node.rs.
    #[allow(dead_code)]
    pub(crate) fn to_pre_image_js(&self) -> FilledTreeResult<String> {
        match &self {
            LeafData::StorageValue(value) => {
                let json_string = serde_json::to_string(&value)?;
                Ok(json_string)
            }

            LeafData::CompiledClassHash(class_hash) => {
                let temp_object_to_json = LeafCompiledClassToSerialise {
                    compiled_class_hash: class_hash.0,
                };
                let json_string = serde_json::to_string(&temp_object_to_json)?;
                Ok(json_string)
            }

            LeafData::StateTreeTuple {
                class_hash: _,
                contract_state_root_hash: _,
                nonce: _,
            } => {
                todo!(
                    "Aviv 11/04 Implement this method, I need
                referrence to tree struct"
                )
            }
        }
    }

    /// Serialises the leaf data into a byte array.
    /// The serialisation is done as follows:
    /// - For storage values: Serialises the value into a 32-byte array.
    /// - For compiled class hashes or state tree tuples: casts the json string
    ///   describing the leaf into a byte vector.
    pub(crate) fn serialise(&self) -> Result<SerialiseNode, FilledTreeError> {
        match &self {
            LeafData::StorageValue(value) => Ok(SerialiseNode::StorageLeaf(value.as_bytes())),

            LeafData::CompiledClassHash(_) => {
                // Serialise the leaf into a JSON, and than to a byte Vector.
                let json = self.to_pre_image_js()?;
                Ok(SerialiseNode::CompiledClassLeaf(
                    json.into_bytes().to_owned(),
                ))
            }

            LeafData::StateTreeTuple {
                class_hash: _,
                contract_state_root_hash: _,
                nonce: _,
            } => {
                // Serialise the leaf into a JSON, and than to a byte Vector.
                let json = self.to_pre_image_js()?;
                Ok(SerialiseNode::CompiledClassLeaf(
                    json.into_bytes().to_owned(),
                ))
            }
        }
    }
}

impl FilledNode<LeafData> {
    /// This method serialises the filled node into a byte vector, where:
    /// - For binary nodes: Concatenates left and right hashes.
    /// - For edge nodes: Concatenates bottom hash, path, and path length.
    /// - For leaf nodes: use leaf.serialise() method.
    #[allow(dead_code, non_snake_case)]
    pub(crate) fn serialise(&self) -> FilledTreeResult<SerialiseNode> {
        match &self.data {
            NodeData::Binary(BinaryData {
                left_hash,
                right_hash,
            }) => {
                let mut serialised: [u8; BINARY_BYTES] = [0; BINARY_BYTES];

                // Serialise left and right hashes to byte arrays.
                let left: [u8; SERIALISE_HASH_BYTES] = left_hash.0.as_bytes();
                let right: [u8; SERIALISE_HASH_BYTES] = right_hash.0.as_bytes();

                // Concatenate left and right hashes.
                serialised[..SERIALISE_HASH_BYTES].copy_from_slice(&left);
                serialised[SERIALISE_HASH_BYTES..].copy_from_slice(&right);
                Ok(SerialiseNode::Binary(serialised))
            }

            NodeData::Edge(EdgeData {
                bottom_hash,
                path_to_bottom,
            }) => {
                // Serialise bottom hash, path, and path length to byte arrays.
                let mut serialised: [u8; EDGE_BYTES] = [0; EDGE_BYTES];
                let bottom: [u8; SERIALISE_HASH_BYTES] = bottom_hash.0.as_bytes();
                let path: [u8; SERIALISE_HASH_BYTES] = path_to_bottom.path.0.as_bytes();
                let length: [u8; 1] = path_to_bottom.length.0.to_be_bytes();

                // Concatenate bottom hash, path, and path length.
                serialised[..SERIALISE_HASH_BYTES].copy_from_slice(&bottom);
                serialised[SERIALISE_HASH_BYTES..(SERIALISE_HASH_BYTES + SERIALISE_HASH_BYTES)]
                    .copy_from_slice(&path);
                serialised[EDGE_BYTES - 1] = length[0];
                Ok(SerialiseNode::Edge(serialised))
            }

            NodeData::Leaf(LeafData) => LeafData.serialise(),
        }
    }
}
