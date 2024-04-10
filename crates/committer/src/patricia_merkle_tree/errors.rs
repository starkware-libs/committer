// TODO(Amos, 01/04/2024): Add error types.
use derive_more::Display;

#[derive(Debug)]
pub(crate) enum OriginalSkeletonTreeError {}

#[derive(Debug)]
pub(crate) enum UpdatedSkeletonTreeError {
    MissingNode,
}

#[derive(thiserror::Error, Debug, Display)]
pub(crate) enum FilledTreeError {
    MissingRoot,
    SerializeError(#[from] serde_json::Error),
}
