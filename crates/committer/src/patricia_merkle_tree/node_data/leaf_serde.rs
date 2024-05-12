use crate::felt::Felt;
use crate::patricia_merkle_tree::filled_tree::node::ClassHash;
use crate::patricia_merkle_tree::filled_tree::node::Nonce;
use crate::patricia_merkle_tree::node_data::leaf::{
    ContractState, LeafDataImpl, SkeletonContractState, UpdatedSkeletonLeafDataImpl,
};
use crate::storage::storage_trait::StoragePrefix;
use crate::storage::storage_trait::StorageValue;

pub trait HasStoragePrefix {
    fn storage_prefix(&self) -> StoragePrefix;
}

pub trait HasSerialize {
    /// Serializes the leaf data into a byte vector.
    fn serialize(&self) -> StorageValue;
}

/// Generates an implementation of the `HasStoragePrefix` trait for a LeafDataImpl and UpdatedSkeletonLeafDataImpl.
/// This trait implementation provides a method to determine the storage prefix based on the variant of the type.
macro_rules! generate_storage_prefix_impl {
    ($leaf_impl:ty) => {
        impl HasStoragePrefix for $leaf_impl {
            fn storage_prefix(&self) -> StoragePrefix {
                match self {
                    Self::StorageValue(_) => StoragePrefix::StorageLeaf,
                    Self::CompiledClassHash(_) => StoragePrefix::CompiledClassLeaf,
                    Self::ContractState(_) => StoragePrefix::StateTreeLeaf,
                }
            }
        }
    };
}

// Apply the implementation to the LeafDataImpl and UpdatedSkeletonLeafDataImpl types.
generate_storage_prefix_impl!(LeafDataImpl);
generate_storage_prefix_impl!(UpdatedSkeletonLeafDataImpl);

/// Serializes the compiled class hash into a JSON string representing the class hash,
/// and converts it into a byte vector for storage.
fn serialize_class_hash(class_hash: &ClassHash) -> StorageValue {
    let json_string = format!(r#"{{"compiled_class_hash": "{}"}}"#, class_hash.0.to_hex());
    StorageValue(json_string.into_bytes())
}

/// Serializes the contract state into a JSON string representing the contract's hash,
/// the root hash of its storage commitment tree (or a default value if not specified), and the nonce.
/// The JSON string is then converted into a byte vector for storage.
fn serialize_contract_state(
    class_hash: &ClassHash,
    root_hash: Felt,
    nonce: &Nonce,
) -> StorageValue {
    // TODO(Aviv, 8/5/2024): Use height from the input.

    let json_string = format!(
        r#"{{"contract_hash": "{}", "storage_commitment_tree": {{"root": {}, "height": 252}}, "nonce": "{}"}}"#,
        class_hash.0.to_fixed_hex_string(),
        root_hash.to_fixed_hex_string(),
        nonce.0.to_hex(),
    );
    StorageValue(json_string.into_bytes())
}

impl HasSerialize for LeafDataImpl {
    /// Serializes the leaf data into a byte vector.
    /// The serialization is done as follows:
    /// - For storage values: serializes the value into a 32-byte vector.
    /// - For compiled class hashes or state tree tuples: creates a  json string
    ///   describing the leaf and cast it into a byte vector.
    fn serialize(&self) -> StorageValue {
        match &self {
            LeafDataImpl::StorageValue(value) => StorageValue(value.to_bytes_be().to_vec()),

            LeafDataImpl::CompiledClassHash(class_hash) => serialize_class_hash(class_hash),

            LeafDataImpl::ContractState(ContractState {
                class_hash,
                storage_root_hash,
                nonce,
            }) => serialize_contract_state(class_hash, storage_root_hash.0, nonce),
        }
    }
}

impl HasSerialize for UpdatedSkeletonLeafDataImpl {
    /// Serializes the leaf data into a byte vector.
    /// The serialization is done as follows:
    /// - For storage values: serializes the value into a 32-byte vector.
    /// - For compiled class hashes or state tree tuples: creates a  json string
    ///   describing the leaf and cast it into a byte vector
    ///   (or a default value if not specified).
    fn serialize(&self) -> StorageValue {
        match &self {
            UpdatedSkeletonLeafDataImpl::StorageValue(value) => {
                StorageValue(value.to_bytes_be().to_vec())
            }

            UpdatedSkeletonLeafDataImpl::CompiledClassHash(class_hash) => {
                serialize_class_hash(class_hash)
            }

            UpdatedSkeletonLeafDataImpl::ContractState(SkeletonContractState {
                class_hash,
                storage_root_hash,
                nonce,
            }) => {
                let root = match storage_root_hash {
                    Some(root_hash) => root_hash.0,
                    None => Felt::ZERO,
                };
                serialize_contract_state(class_hash, root, nonce)
            }
        }
    }
}
