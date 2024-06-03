use crate::patricia_merkle_tree::node_data::errors::{EdgePathError, PathToBottomError};
use crate::storage::storage_trait::StorageKey;

use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("The key {0:?} does not exist in storage.")]
    MissingKey(StorageKey),
}

#[derive(thiserror::Error, Debug)]
pub enum SerializationError {
    #[error("Serialize error: {0}")]
    SerializeError(#[from] serde_json::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DeserializationError {
    #[error("There is a key duplicate at {0} mapping.")]
    KeyDuplicate(String),
    #[error("Couldn't read and parse the given input JSON: {0}")]
    ParsingError(#[from] serde_json::Error),
    #[error("Illegal EdgePath: {0}")]
    EdgePathError(#[from] EdgePathError),
    #[error("Illegal PathToBottom: {0}")]
    PathToBottomError(#[from] PathToBottomError),
}
