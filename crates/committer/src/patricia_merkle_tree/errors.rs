// TODO(Amos, 01/04/2024): Add error types.
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum OriginalSkeletonTreeError {
    // TODO(Nimrod, 5/5/2024): If possible, divide Deserialization to more specific types of errors.
    Deserialization(String),
    StorageRead,
}

#[derive(Debug)]
pub(crate) enum UpdatedSkeletonTreeError {
    MissingNode,
}

pub(crate) enum FilledTreeError {
    MissingRoot,
}
