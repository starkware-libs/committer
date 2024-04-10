
use crate::patricia_merkle_tree::types::{EdgeData, LeafDataTrait};
use crate::{hash::types::HashOutput, types::Felt};
use crate::types::FeltTrait;
use crate::patricia_merkle_tree::serialised_node::{
                                                    SerialiseNode,
                                                    LeafCompiledClassToSerialise,
                                                    SERIALISE_HASH_SIZE,
                                                    BINARY_SIZE,
                                                    EDGE_SIZE,
                                                };
use serde_json;

// TODO(Nimrod, 1/6/2024): Swap to starknet-types-core types once implemented.
#[allow(dead_code)]
pub(crate) struct ClassHash(pub Felt);
#[allow(dead_code)]
pub(crate) struct Nonce(pub Felt);

#[allow(dead_code)]
/// A node in a Patricia-Merkle tree which was modified during an update.
pub(crate) struct FilledNode<L: LeafDataTrait> {
    hash: HashOutput,
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

impl LeafData{

    /// Serialises the leaf data into a JSON string,
    /// using temp structs which define in serialised_node.rs.
    #[allow(dead_code)]
    pub(crate) fn to_pre_image_js(&self) -> String {

        match &self {
            LeafData::StorageValue(_value) => {
                panic!("Storage leaf should not be serialised as JSON.")
            }
            LeafData::CompiledClassHash(class_hash) => {
                let temp_object_to_json = LeafCompiledClassToSerialise {
                    compiled_class_hash: class_hash.0,
                };
                serde_json::to_string(&temp_object_to_json).unwrap()
            }

            LeafData::StateTreeTuple{class_hash: _, contract_state_root_hash: _, nonce: _} => {
                todo!("Aviv 11/04 Implement this method, I need
                referrence to tree struct")
                }
            }
    }



    /// Serialises the leaf data into a byte array.
    /// The serialisation is done as follows:
    /// - For storage values: Serialises the value into a 32-byte array.
    /// - For compiled class hashes: cast the json describe the leaf into a byte vector.
    /// - For state tree tuples: cast the json describe the leaf into a byte vector.
    pub(crate) fn serialise(&self) -> SerialiseNode {
        match &self {
            LeafData::StorageValue(value) => {
                SerialiseNode::StorageLeaf(value.to_bytes())
            }

            LeafData::CompiledClassHash(_) => {
                // Serialise the leaf into a JSON, and than to a byte Vector.
                let json_bytes = &self.to_pre_image_js().into_bytes();
                SerialiseNode::CompiledClassLeaf(json_bytes.to_owned())
            }

            LeafData::StateTreeTuple{ class_hash: _, contract_state_root_hash: _, nonce: _ } => {
                // Serialise the leaf into a JSON, and than to a byte Vector.
                let json_bytes = &self.to_pre_image_js().into_bytes();
                SerialiseNode::StateTreeLeaf(json_bytes.to_owned())
            }
        }
    }
}

impl FilledNode<LeafData>{

    /// This method serialises the filled node into a byte vector, where:
    /// - For binary nodes: Concatenates left and right hashes.
    /// - For edge nodes: Concatenates bottom hash, path, and path length.
    /// - For leaf nodes: use leaf.serialise() method.
    #[allow(dead_code,non_snake_case)]
    pub(crate) fn serialise(&self,) -> SerialiseNode {

        match &self.data {
            NodeData::Binary(BinaryData {left_hash, right_hash}) => {
                let mut serialised: [u8; BINARY_SIZE] = [0; BINARY_SIZE];

                // Serialise left and right hashes to byte arrays.
                let left: [u8; SERIALISE_HASH_SIZE] = left_hash.0.to_bytes();
                let right: [u8; SERIALISE_HASH_SIZE] = right_hash.0.to_bytes();

                // Concatenate left and right hashes.
                serialised[..SERIALISE_HASH_SIZE].copy_from_slice(&left);
                serialised[SERIALISE_HASH_SIZE..].copy_from_slice(&right);
                SerialiseNode::Binary(serialised)
            }


            NodeData::Edge(EdgeData {bottom_hash, path_to_bottom}) => {
                // Serialise bottom hash, path, and path length to byte arrays.
                let mut serialised: [u8; EDGE_SIZE] = [0; EDGE_SIZE];
                let bottom: [u8; SERIALISE_HASH_SIZE] = bottom_hash.0.to_bytes();
                let path: [u8; SERIALISE_HASH_SIZE] = path_to_bottom.path.0.to_bytes();
                let length: [u8; 1] = path_to_bottom.length.0.to_be_bytes();

                // Concatenate bottom hash, path, and path length.
                serialised[..SERIALISE_HASH_SIZE].copy_from_slice(&bottom);
                serialised[SERIALISE_HASH_SIZE..(SERIALISE_HASH_SIZE+SERIALISE_HASH_SIZE)].copy_from_slice(&path);
                serialised[EDGE_SIZE-1] = length[0];
                SerialiseNode::Edge(serialised)
            }

            NodeData::Leaf(LeafData) => {
                LeafData.serialise()
            }
        }
    }
}
