use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::node::{ClassHash, CompiledClassHash, Nonce};
use crate::patricia_merkle_tree::node_data::errors::{LeafError, LeafResult};
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::db_object::DBObject;
use strum_macros::{EnumDiscriminants, EnumIter};

pub trait LeafData: Clone + Sync + Send + DBObject {
    /// Returns true if leaf is empty.
    fn is_empty(&self) -> bool;

    /// Creates a leaf.
    // Use explicit desugaring of `async fn` to allow adding trait bounds to the return type, see
    // https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits
    // for details.
    fn create(
        index: &NodeIndex,
        leaf_modifications: Arc<LeafModifications<LeafDataImpl>>,
    ) -> impl Future<Output = LeafResult<Self>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractState {
    pub nonce: Nonce,
    pub storage_root_hash: HashOutput,
    pub class_hash: ClassHash,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "testing"), derive(EnumDiscriminants))]
#[cfg_attr(any(test, feature = "testing"), strum_discriminants(derive(EnumIter)))]
pub enum LeafDataImpl {
    StorageValue(Felt),
    CompiledClassHash(CompiledClassHash),
    ContractState(ContractState),
}

impl LeafData for LeafDataImpl {
    fn is_empty(&self) -> bool {
        match self {
            LeafDataImpl::StorageValue(value) => *value == Felt::ZERO,
            LeafDataImpl::CompiledClassHash(class_hash) => class_hash.0 == Felt::ZERO,
            LeafDataImpl::ContractState(contract_state) => {
                contract_state.nonce.0 == Felt::ZERO
                    && contract_state.class_hash.0 == Felt::ZERO
                    && contract_state.storage_root_hash.0 == Felt::ZERO
            }
        }
    }

    async fn create(
        index: &NodeIndex,
        leaf_modifications: Arc<LeafModifications<LeafDataImpl>>,
    ) -> LeafResult<Self> {
        let leaf_data = leaf_modifications
            .get(index)
            .ok_or(LeafError::MissingLeafModificationData(*index))?
            .clone();
        Ok(leaf_data)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SkeletonLeaf {
    Zero,
    NonZero,
}

impl SkeletonLeaf {
    pub(crate) fn is_zero(&self) -> bool {
        self == &Self::Zero
    }
}

impl From<Felt> for SkeletonLeaf {
    fn from(value: Felt) -> Self {
        if value == Felt::ZERO {
            Self::Zero
        } else {
            Self::NonZero
        }
    }
}

pub type LeafModifications<L> = HashMap<NodeIndex, L>;
