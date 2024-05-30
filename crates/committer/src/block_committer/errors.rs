use thiserror::Error;

use crate::forest_errors::ForestError;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub(crate) enum BlockCommitmentError {
    #[error(transparent)]
    ForestError(#[from] ForestError),
}
