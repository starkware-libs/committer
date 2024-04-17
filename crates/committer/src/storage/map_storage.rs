use std::collections::HashMap;

use crate::storage::storage_trait::{Storage, StorageKey, StorageValue};

pub(crate) struct MapStorage {
    storage: HashMap<StorageKey, StorageValue>,
}

impl Storage for MapStorage {
    fn get(&self, key: &StorageKey) -> Option<&StorageValue> {
        self.storage.get(key)
    }

    fn set(&mut self, key: StorageKey, value: StorageValue) -> Option<StorageValue> {
        self.storage.insert(key, value)
    }

    fn mget(&self, keys: &[StorageKey]) -> Vec<Option<&StorageValue>> {
        keys.iter().map(|key| self.get(key)).collect::<Vec<_>>()
    }

    fn mset(&mut self, key_to_value: HashMap<StorageKey, StorageValue>) {
        self.storage.extend(key_to_value);
    }

    fn delete(&mut self, key: &StorageKey) -> Option<StorageValue> {
        self.storage.remove(key)
    }
}