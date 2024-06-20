use crate::patricia_merkle_tree::node_data::errors::{EdgePathError, PathToBottomError};
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::storage_trait::StoragePrefix;

use serde_json;
use starknet_types_core::felt::FromStrError;
use thiserror::Error;

use super::storage_trait::StorageKey;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("The key {0:?} does not exist in storage at node index {1:?}.")]
    MissingKey(StorageKey, NodeIndex),
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
    #[error("The key {0} unexpectedly doesn't exist.")]
    NonExistingKey(String),
    #[error(transparent)]
    ParsingError(#[from] serde_json::Error),
    #[error(transparent)]
    EdgePathError(#[from] EdgePathError),
    #[error(transparent)]
    PathToBottomError(#[from] PathToBottomError),
    #[error("Unexpected prefix ({0:?}) variant when deserializing a leaf.")]
    LeafPrefixError(StoragePrefix),
    #[error(transparent)]
    StringConversionError(#[from] std::str::Utf8Error),
    #[error(transparent)]
    FeltParsingError(#[from] FromStrError),
    #[error("Encountered an invalid type when deserializing a leaf.")]
    LeafTypeError,
}
