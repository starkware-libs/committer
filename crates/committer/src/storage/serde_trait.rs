use crate::storage::errors::SerdeError;
use crate::storage::storage_trait::{StorageError, StorageKey, StorageValue};

pub(crate) trait serde_trait {
    /// Serializes the given value.
    fn serialize(&self) -> Result<StorageValue, SerdeError::Serialize>;
    /// Deserializes the given value.
    fn deserialize(vale: StorageValue) -> Result<Self, SerdeError::Deserialize>;
}
