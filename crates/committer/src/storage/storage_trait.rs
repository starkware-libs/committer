use crate::types::Felt;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StorageKey(pub(crate) Vec<u8>);

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StorageValue(pub(crate) Vec<u8>);

pub(crate) trait Storage {
    /// Returns value from storage, if it exists.
    fn get(&self, key: &StorageKey) -> Option<&StorageValue>;

    /// Sets value in storage. If key already exists, its value is overwritten and the old value is
    /// returned.
    fn set(&mut self, key: StorageKey, value: StorageValue) -> Option<StorageValue>;

    /// Returns values from storage in same order of given keys. Value is None for keys that do not
    /// exist.
    fn mget(&self, keys: &[StorageKey]) -> Vec<Option<&StorageValue>>;

    /// Sets values in storage.
    fn mset(&mut self, key_to_value: HashMap<StorageKey, StorageValue>);

    /// Deletes value from storage and returns its value if it exists. Returns None if not.
    fn delete(&mut self, key: &StorageKey) -> Option<StorageValue>;
}

pub(crate) enum StoragePrefix {
    InnerNode,
    StorageLeaf,
    StateTreeLeaf,
    CompiledClassLeaf,
}

/// Describes a storage prefix as used in Aerospike DB.
impl StoragePrefix {
    pub(crate) fn to_bytes(&self) -> &[u8] {
        match self {
            Self::InnerNode => b"patricia_node",
            Self::StorageLeaf => b"starknet_storage_leaf",
            Self::StateTreeLeaf => b"contract_state",
            Self::CompiledClassLeaf => b"contract_class_leaf",
        }
    }
}

impl StorageKey {
    pub(crate) fn with_prefix(&self, prefix: StoragePrefix) -> Self {
        let mut prefix = [prefix.to_bytes().to_vec(), b":".to_vec()].concat();
        prefix.extend(&self.0);
        StorageKey(prefix)
    }

    pub(crate) fn from_prefix_and_suffix(prefix: StoragePrefix, suffix: &[u8]) -> Self {
        Self(suffix.to_vec()).with_prefix(prefix)
    }
}

impl From<Felt> for StorageKey {
    fn from(value: Felt) -> Self {
        StorageKey(value.to_bytes_be().to_vec())
    }
}
