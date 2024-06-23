use crate::block_committer::input::StarknetStorageValue;
use crate::patricia_merkle_tree::filled_tree::node::CompiledClassHash;
use crate::patricia_merkle_tree::node_data::leaf::{ContractState, LeafData, LeafModifications};
use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTreeResult;
use crate::patricia_merkle_tree::types::NodeIndex;

/// Configures the creation of an original skeleton tree.
pub(crate) trait OriginalSkeletonTreeConfig<L: LeafData> {
    /// Configures whether modified leaves should be compared to the previous leaves and log out a
    /// warning when encountering a trivial modification.
    fn compare_modified_leaves(&self) -> bool;

    /// Compares the previous leaf to the modificated and returns true iff they are equal.
    fn compare_leaf(
        &self,
        index: &NodeIndex,
        previous_leaf: &L,
    ) -> OriginalSkeletonTreeResult<bool>;
}

pub(crate) struct OriginalSkeletonStorageTrieConfig<'a> {
    modifications: &'a LeafModifications<StarknetStorageValue>,
    compare_modified_leaves: bool,
}

impl OriginalSkeletonTreeConfig<StarknetStorageValue> for OriginalSkeletonStorageTrieConfig<'_> {
    fn compare_modified_leaves(&self) -> bool {
        self.compare_modified_leaves
    }

    fn compare_leaf(
        &self,
        index: &NodeIndex,
        previous_leaf: &StarknetStorageValue,
    ) -> OriginalSkeletonTreeResult<bool> {
        let new_leaf = self
            .modifications
            .get(index)
            .ok_or(OriginalSkeletonTreeError::ReadModificationsError(*index))?;
        Ok(new_leaf == previous_leaf)
    }
}

impl<'a> OriginalSkeletonStorageTrieConfig<'a> {
    pub(crate) fn new(
        modifications: &'a LeafModifications<StarknetStorageValue>,
        compare_modified_leaves: bool,
    ) -> Self {
        Self {
            modifications,
            compare_modified_leaves,
        }
    }
}

pub(crate) struct OriginalSkeletonClassesTrieConfig<'a> {
    modifications: &'a LeafModifications<CompiledClassHash>,
    compare_modified_leaves: bool,
}

impl OriginalSkeletonTreeConfig<CompiledClassHash> for OriginalSkeletonClassesTrieConfig<'_> {
    fn compare_modified_leaves(&self) -> bool {
        self.compare_modified_leaves
    }

    fn compare_leaf(
        &self,
        index: &NodeIndex,
        previous_leaf: &CompiledClassHash,
    ) -> OriginalSkeletonTreeResult<bool> {
        let new_leaf = self
            .modifications
            .get(index)
            .ok_or(OriginalSkeletonTreeError::ReadModificationsError(*index))?;
        Ok(new_leaf == previous_leaf)
    }
}

impl<'a> OriginalSkeletonClassesTrieConfig<'a> {
    pub(crate) fn new(
        modifications: &'a LeafModifications<CompiledClassHash>,
        compare_modified_leaves: bool,
    ) -> Self {
        Self {
            modifications,
            compare_modified_leaves,
        }
    }
}

pub(crate) struct OriginalSkeletonContractsTrieConfig;

impl OriginalSkeletonTreeConfig<ContractState> for OriginalSkeletonContractsTrieConfig {
    fn compare_modified_leaves(&self) -> bool {
        false
    }

    fn compare_leaf(
        &self,
        _index: &NodeIndex,
        _previous_leaf: &ContractState,
    ) -> OriginalSkeletonTreeResult<bool> {
        Ok(false)
    }
}

impl OriginalSkeletonContractsTrieConfig {
    pub(crate) fn new() -> Self {
        Self
    }
}
