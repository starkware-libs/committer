use std::collections::HashMap;
use std::future::Future;

use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::node::{ClassHash, CompiledClassHash, Nonce};
use crate::patricia_merkle_tree::filled_tree::tree::FilledTreeResult;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::db_object::DBObject;
use strum_macros::{EnumDiscriminants, EnumIter};

use crate::patricia_merkle_tree::node_data::inner_node::NodeData;

pub trait LeafData: Clone + Sync + Send + DBObject {
    /// Returns true if leaf is empty.
    fn is_empty(&self) -> bool;

    /// Returns / computes the NodeData.
    // Use explicit desugaring of `async fn` to allow adding trait bounds to the return type, see
    // https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits
    // for details.
    fn get_node_data(
        index: &NodeIndex,
        leaf_modifications: &LeafModifications<Self>,
    ) -> impl Future<Output = FilledTreeResult<NodeData<Self>, Self>>;
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractState {
    pub nonce: Nonce,
    pub storage_root_hash: HashOutput,
    pub class_hash: ClassHash,
}

#[allow(dead_code)]
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

    async fn get_node_data(
        index: &NodeIndex,
        leaf_modifications: &LeafModifications<Self>,
    ) -> FilledTreeResult<NodeData<Self>, Self> {
        let leaf_data = leaf_modifications
            .get(index)
            .ok_or(FilledTreeError::<LeafDataImpl>::MissingDataForUpdate(
                *index,
            ))?
            .clone();
        if leaf_data.is_empty() {
            return Err(FilledTreeError::<LeafDataImpl>::DeletedLeafInSkeleton(
                *index,
            ));
        }
        Ok(NodeData::Leaf(leaf_data))
    }
}

#[allow(dead_code)]
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

pub(crate) type LeafModifications<L> = HashMap<NodeIndex, L>;
