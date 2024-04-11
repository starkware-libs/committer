use crate::{
    patricia_merkle_tree::{
        errors::OriginalSkeletonTreeError,
        original_skeleton_tree::OriginalSkeletonTreeResult,
        types::{EdgePath, EdgePathLength, PathToBottom},
    },
    storage::errors::StorageError,
    types::Felt,
};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash)]
pub(crate) struct StorageKey(pub Vec<u8>);

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct StorageValue(pub Vec<u8>);

pub(crate) trait Storage {
    /// Returns value from storage, if it exists.
    fn get(&self, key: &StorageKey) -> Option<StorageValue>;
    /// Sets value in storage.
    fn set(&mut self, key: &StorageKey, value: &StorageValue);
    /// Returns values from storage in same order of given keys. If key does not exist,
    /// value is None.
    fn mget(&self, keys: &[StorageKey]) -> [Option<StorageValue>];
    /// Sets values in storage.
    fn mset(&mut self, key_to_value: &HashMap<StorageKey, StorageValue>);
    /// Deletes value from storage. Returns error if key does not exist.
    fn delete(&mut self, key: &StorageKey) -> Result<(), StorageError>;
}

impl StorageValue {
    pub(crate) fn deserialize_binary_node(
        &self,
    ) -> OriginalSkeletonTreeResult<(StorageKey, StorageKey)> {
        if !self.is_binary_node() {
            return Err(OriginalSkeletonTreeError::Deserializtion(
                "Could not deserailize a binary node when building the original skeleton tree."
                    .to_string(),
            ));
        }
        Ok((
            StorageKey(self.0[..32].to_vec()),
            StorageKey(self.0[32..].to_vec()),
        ))
    }

    pub(crate) fn deserialize_edge_node(
        &self,
    ) -> OriginalSkeletonTreeResult<(StorageKey, PathToBottom)> {
        if !self.is_edge_node() {
            return Err(OriginalSkeletonTreeError::Deserializtion(
                "Could not deserailize an edge node when building the original skeleton tree."
                    .to_string(),
            ));
        }
        let bottom_hash = StorageKey(self.0[..32].to_vec());
        let path = EdgePath(Felt::from_bytes_be_slice(&self.0[32..64]));
        let length = EdgePathLength(self.0[64]);
        Ok((bottom_hash, PathToBottom { path, length }))
    }

    pub(crate) fn is_binary_node(&self) -> bool {
        // TODO(Nimrod, 30/4/2024): Compare to a constant value once Aviv's PR is merged.
        self.0.len() == 64
    }

    pub(crate) fn is_edge_node(&self) -> bool {
        // TODO(Nimrod, 30/4/2024): Compare to a constant value once Aviv's PR is merged.
        self.0.len() == 65
    }
}

impl StorageKey {
    pub(crate) fn with_patricia_prefix(&self) -> Self {
        let mut prefix = "patricia_node:".as_bytes().to_vec();
        prefix.extend(&self.0);
        StorageKey(prefix)
    }
}

impl From<Felt> for StorageKey {
    fn from(value: Felt) -> Self {
        StorageKey(value.to_bytes_be().to_vec())
    }
}
