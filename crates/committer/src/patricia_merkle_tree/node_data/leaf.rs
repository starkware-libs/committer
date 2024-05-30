use std::collections::HashMap;
use std::future::Future;

use crate::block_committer::input::StarknetStorageValue;
use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::node::{ClassHash, CompiledClassHash, Nonce};
use crate::patricia_merkle_tree::filled_tree::tree::FilledTreeResult;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::db_object::DBObject;

use crate::patricia_merkle_tree::node_data::inner_node::NodeData;

pub trait LeafData: Clone + Default + Sync + Send + DBObject {
    /// Returns true if leaf is empty.
    fn is_empty(&self) -> bool;

    /// Returns / computes the NodeData.
    // Use explicit desugaring of `async fn` to allow adding trait bounds to the return type, see
    // https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html#async-fn-in-public-traits
    // for details.
    fn get_node_data(
        index: &NodeIndex,
        leaf_modifications: &LeafModifications<Self>,
    ) -> impl Future<Output = FilledTreeResult<NodeData<Self>, Self>> + Send;
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContractState {
    pub nonce: Nonce,
    pub storage_root_hash: HashOutput,
    pub class_hash: ClassHash,
}

fn get_node_data_common<L: LeafData>(
    index: &NodeIndex,
    leaf_modifications: &LeafModifications<L>,
) -> FilledTreeResult<NodeData<L>, L> {
    let leaf_data = leaf_modifications
        .get(index)
        .ok_or(FilledTreeError::<L>::MissingDataForUpdate(*index))?
        .clone();
    if leaf_data.is_empty() {
        return Err(FilledTreeError::<L>::DeletedLeafInSkeleton(*index));
    }
    Ok(NodeData::Leaf(leaf_data))
}

impl LeafData for StarknetStorageValue {
    fn is_empty(&self) -> bool {
        self.0 == Felt::ZERO
    }

    async fn get_node_data(
        index: &NodeIndex,
        leaf_modifications: &LeafModifications<Self>,
    ) -> FilledTreeResult<NodeData<Self>, Self> {
        get_node_data_common(index, leaf_modifications)
    }
}

impl LeafData for CompiledClassHash {
    fn is_empty(&self) -> bool {
        self.0 == Felt::ZERO
    }

    async fn get_node_data(
        index: &NodeIndex,
        leaf_modifications: &LeafModifications<Self>,
    ) -> FilledTreeResult<NodeData<Self>, Self> {
        get_node_data_common(index, leaf_modifications)
    }
}

impl LeafData for ContractState {
    fn is_empty(&self) -> bool {
        self.nonce.0 == Felt::ZERO
            && self.class_hash.0 == Felt::ZERO
            && self.storage_root_hash.0 == Felt::ZERO
    }

    async fn get_node_data(
        index: &NodeIndex,
        leaf_modifications: &LeafModifications<Self>,
    ) -> FilledTreeResult<NodeData<Self>, Self> {
        get_node_data_common(index, leaf_modifications)
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
