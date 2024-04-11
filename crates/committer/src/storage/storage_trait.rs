use crate::{storage::errors::StorageError, types::Felt};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash)]
pub(crate) struct StorageKey(pub Vec<u8>);

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct StorageValue(pub Vec<u8>);

pub(crate) trait Storage {
    /// Returns value from storage, if it exists.
    fn get(&self, key: &StorageKey) -> Option<StorageValue>;
    /// Sets value in storage.
    fn set(&mut self, key: &StorageKey, value: &StorageValue);
    /// Returns values from storage in same order of given keys. If key does not exist,
    /// value is None.
    fn mget(&self, keys: &[StorageKey]) -> [Option<StorageValue>];
    /// Sets values in storage.
    fn mset(&mut self, key_to_value: &HashMap<StorageKey, StorageValue>);
    /// Deletes value from storage. Returns error if key does not exist.
    fn delete(&mut self, key: &StorageKey) -> Result<(), StorageError>;
}

impl StorageKey {
    pub(crate) fn with_prefix(&self, prefix: StoragePrefix) -> Self {
        let mut prefix = prefix.to_bytes().to_vec();
        prefix.extend(&self.0);
        StorageKey(prefix)
    }
}

impl From<Felt> for StorageKey {
    fn from(value: Felt) -> Self {
        StorageKey(value.to_bytes_be().to_vec())
    }
}

#[allow(dead_code)]
pub(crate) enum StoragePrefix {
    PatriciaNode,
}
#[allow(dead_code)]
impl StoragePrefix {
    pub(crate) fn to_bytes(&self) -> &[u8] {
        match self {
            Self::PatriciaNode => "patricia_node:".as_bytes(),
        }
    }
}
