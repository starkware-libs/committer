use thiserror::Error;

use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::storage_trait::{StorageKey, StorageValue};

// TODO(Amos, 01/04/2024): Add error types.
#[allow(dead_code)]
#[derive(Debug, Error)]
pub(crate) enum OriginalSkeletonTreeError {
    #[error(
        "Falied to deserialize the storage value: {0:?} while building the original skeleton tree."
    )]
    Deserialization(StorageValue),
    #[error(
        "Unable to read from storage the storage key: {0:?} while building the \
         original skeleton tree."
    )]
    StorageRead(StorageKey),
    #[error("`fetch_nodes` method encountered unexpectedally a leaf node at index: {0:?}")]
    LeafEncountered(NodeIndex),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum UpdatedSkeletonTreeError {
    MissingNode,
    PoisonedLock(String),
    NonDroppedPointer(String),
}

#[derive(Debug)]
pub(crate) enum FilledTreeError {
    MissingRoot,
}
