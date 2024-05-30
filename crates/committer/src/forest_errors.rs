use crate::block_committer::input::{ContractAddress, StarknetStorageValue};
use crate::patricia_merkle_tree::filled_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::node::CompiledClassHash;
use crate::patricia_merkle_tree::node_data::leaf::ContractState;
use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;
use crate::patricia_merkle_tree::updated_skeleton_tree::errors::UpdatedSkeletonTreeError;

use thiserror::Error;
use tokio::task::JoinError;

pub(crate) type ForestResult<T> = Result<T, ForestError>;

#[derive(Debug, Error)]
pub(crate) enum ForestError {
    #[error(transparent)]
    OriginalSkeleton(#[from] OriginalSkeletonTreeError),
    #[error(transparent)]
    UpdatedSkeleton(#[from] UpdatedSkeletonTreeError),
    #[error(transparent)]
    CompiledClassHashTrie(#[from] FilledTreeError<CompiledClassHash>),
    #[error(transparent)]
    StorageTrie(#[from] FilledTreeError<StarknetStorageValue>),
    #[error(transparent)]
    ContractTrie(#[from] FilledTreeError<ContractState>),
    #[error("Missing input: Couldn't find the storage trie's current state of address {0:?}")]
    MissingContractCurrentState(ContractAddress),
    #[error("Can't build storage trie's updated skeleton, because there is no original skeleton at address {0:?}")]
    MissingOriginalSkeleton(ContractAddress),
    #[error("Can't fill storage trie, because there is no updated skeleton at address {0:?}")]
    MissingUpdatedSkeleton(ContractAddress),
    #[error(transparent)]
    JoinError(#[from] JoinError),
}
