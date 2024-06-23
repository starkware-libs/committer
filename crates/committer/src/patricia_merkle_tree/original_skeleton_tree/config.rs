use crate::block_committer::input::StarknetStorageValue;
use crate::patricia_merkle_tree::filled_tree::node::CompiledClassHash;
use crate::patricia_merkle_tree::node_data::leaf::{ContractState, LeafData, LeafModifications};

/// Configures the creation of an original skeleton tree.
pub(crate) trait OriginalSkeletonTreeConfig<L: LeafData> {
    /// Configures wether modified leaves should be compared to the previous leaves and log out a
    /// warning when encountering a trivial modification.
    fn compare_modified_leaves(&self) -> bool;

    /// Returns the modifications in order to compare them.
    fn get_modifications(&self) -> &LeafModifications<L>;
}

pub(crate) struct OriginalSkeletonStorageTrieConfig<'a> {
    modifications: &'a LeafModifications<StarknetStorageValue>,
    compare_modified_leaves: bool,
}

impl OriginalSkeletonTreeConfig<StarknetStorageValue> for OriginalSkeletonStorageTrieConfig<'_> {
    fn compare_modified_leaves(&self) -> bool {
        self.compare_modified_leaves
    }

    fn get_modifications(&self) -> &LeafModifications<StarknetStorageValue> {
        self.modifications
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

    fn get_modifications(&self) -> &LeafModifications<CompiledClassHash> {
        self.modifications
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

pub(crate) struct OriginalSkeletonContractsTrieConfig<'a>(&'a LeafModifications<ContractState>);

impl OriginalSkeletonTreeConfig<ContractState> for OriginalSkeletonContractsTrieConfig<'_> {
    fn compare_modified_leaves(&self) -> bool {
        false
    }

    fn get_modifications(&self) -> &LeafModifications<ContractState> {
        self.0
    }
}

impl<'a> OriginalSkeletonContractsTrieConfig<'a> {
    pub(crate) fn new(modifications: &'a LeafModifications<ContractState>) -> Self {
        Self(modifications)
    }
}
