use thiserror::Error;

use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;

use crate::block_committer::types::ContractAddress;
#[allow(dead_code)]
#[derive(Debug, Error)]
pub(crate) enum BlockCommitmentError {
    #[error("Failed to build the original skeleton tree while committing the block.\n {0:?}")]
    BuildingOriginalSkeletonTree(#[from] OriginalSkeletonTreeError),
    #[error("Failed to commit lower tree at address: {0:?}")]
    LowerTreeCommitmentError(ContractAddress),
}
