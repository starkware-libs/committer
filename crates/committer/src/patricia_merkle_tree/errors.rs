// TODO(Amos, 01/04/2024): Add error types.
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum OriginalSkeletonTreeError {
    Deserializtion(String),
    StorageRead,
}

#[derive(Debug)]
pub(crate) enum UpdatedSkeletonTreeError {
    MissingNode,
}

pub(crate) enum FilledTreeError {
    MissingRoot,
}
