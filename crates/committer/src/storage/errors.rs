use std::fmt::Debug;

use crate::storage::storage_trait::StorageKey;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub(crate) enum StorageError {
    #[error("The key {:?} does not exist in storage.", 0)]
    KeyNotExist(StorageKey),
}
