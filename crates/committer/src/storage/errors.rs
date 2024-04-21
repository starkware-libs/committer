use crate::storage::storage_trait::StorageKey;
use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub(crate) enum StorageError {
    #[error("The key {0:?} does not exist in storage.")]
    MissingKey(StorageKey),
}

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub(crate) enum SerializationError {
    #[error("Failed to deserialize the storage value")]
    DeserializeError,
    #[error("Serialize error: {0}")]
    SerializeError(#[from] serde_json::Error),
}
