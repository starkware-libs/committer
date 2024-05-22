use std::fmt::Debug;
use committer::patricia_merkle_tree::node_data::leaf::LeafData;
use committer::patricia_merkle_tree::filled_tree::errors::FilledTreeError;


#[derive(thiserror::Error, Debug)]
pub(crate) enum FilledForestError<L: LeafData> {
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    #[error(transparent)]
    MissingRoot(#[from] FilledTreeError<L>),
}
