use std::sync::Arc;

use committer::{felt::Felt, hash::hash_trait::HashOutput, patricia_merkle_tree::{node_data::{errors::LeafResult, leaf::{Leaf, LeafModifications}}, types::NodeIndex}};
use crate::{block_committer::input::StarknetStorageValue, starknet_patricia_merkle_tree::node::{ClassHash, CompiledClassHash, Nonce}};
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContractState {
    pub nonce: Nonce,
    pub storage_root_hash: HashOutput,
    pub class_hash: ClassHash,
}

impl Leaf for StarknetStorageValue {
    fn is_empty(&self) -> bool {
        self.0 == Felt::ZERO
    }

    async fn create(
        index: &NodeIndex,
        leaf_modifications: Arc<LeafModifications<Self>>,
    ) -> LeafResult<Self> {
        Self::from_modifications(index, leaf_modifications)
    }
}

impl Leaf for CompiledClassHash {
    fn is_empty(&self) -> bool {
        self.0 == Felt::ZERO
    }

    async fn create(
        index: &NodeIndex,
        leaf_modifications: Arc<LeafModifications<Self>>,
    ) -> LeafResult<Self> {
        Self::from_modifications(index, leaf_modifications)
    }
}

impl Leaf for ContractState {
    fn is_empty(&self) -> bool {
        self.nonce.0 == Felt::ZERO
            && self.class_hash.0 == Felt::ZERO
            && self.storage_root_hash.0 == Felt::ZERO
    }

    async fn create(
        index: &NodeIndex,
        leaf_modifications: Arc<LeafModifications<Self>>,
    ) -> LeafResult<Self> {
        Self::from_modifications(index, leaf_modifications)
    }
}
