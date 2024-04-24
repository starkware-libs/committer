use crate::storage::errors::{DeserializationError, SerializationError};
use crate::storage::storage_trait::{StorageKey, StorageValue};

pub(crate) trait Serializable {
    /// Serializes the given value.
    fn serialize(&self) -> Result<StorageValue, SerializationError>;
    /// Returns the key used to store self in storage.
    fn db_key(&self) -> StorageKey;
    /// Returns a `StorageKey` from a prefix and a suffix.
    fn create_db_key(prefix: &[u8], suffix: &[u8]) -> StorageKey {
        StorageKey([prefix.to_vec(), b":".to_vec(), suffix.to_vec()].concat())
    }
}

pub(crate) trait Deserializable: Sized {
    /// Deserializes the given value.
    fn deserialize(key: &StorageKey, value: &StorageValue) -> Result<Self, DeserializationError>
    where
        Self: Sized;
}
