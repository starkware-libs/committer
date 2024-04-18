use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct StorageKey(pub(crate) Vec<u8>);

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
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

/// Returns a `StorageKey` from a prefix and a suffix.
pub(crate) fn db_key_from_suffix(prefix: &[u8], suffix: &[u8]) -> StorageKey {
    StorageKey([prefix.to_vec(), b":".to_vec(), suffix.to_vec()].concat())
}
