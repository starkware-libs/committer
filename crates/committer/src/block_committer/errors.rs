use thiserror::Error;

use crate::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub(crate) enum BlockCommitmentError {
    #[error(transparent)]
    BuildingOriginalSkeletonTree(#[from] OriginalSkeletonTreeError),
    #[error("Failed to commit lower tree at address: {0:?}")]
    LowerTreeCommitmentError(#[from] LowerTreeCommitmentError),
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub(crate) enum LowerTreeCommitmentError {
    #[error("Failed to build the original skeleton at address: {0:?}")]
    BuildingOriginalSkeletonTree(#[from] OriginalSkeletonTreeError),
}
