// TODO(Amos, 01/04/2024): Add error types.
#[derive(Debug)]
pub(crate) enum CurrentSkeletonTreeError {}

#[derive(Debug)]
pub(crate) enum UpdatedSkeletonTreeError {
    MissingNode,
}

pub(crate) enum FilledTreeError {
    MissingRoot,
}
