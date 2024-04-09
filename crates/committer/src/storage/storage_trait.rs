use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct StorageKey<'a>(&'a [u8]);

#[allow(dead_code)]
pub(crate) struct StorageValue<'a>(&'a [u8]);

pub(crate) trait Storage {
    /// Returns value from storage, if it exists.
    fn get(&self, key: StorageKey<'_>) -> Option<StorageValue<'_>>;
    /// Sets value in storage.
    fn set(&mut self, key: StorageKey<'_>, value: StorageValue<'_>);
    /// Returns values from storage in same order of given keys. If key does not exist,
    /// value is None.
    fn mget(&self, keys: Vec<StorageKey<'_>>) -> Vec<Option<StorageValue<'_>>>;
    /// Sets values in storage.
    fn mset(&mut self, key_to_value: &HashMap<StorageKey<'_>, StorageValue<'_>>);
    /// Deletes value from storage.
    fn delete(&mut self, key: StorageKey<'_>);
}
