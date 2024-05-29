use std::fmt::Debug;
use thiserror::Error;

use crate::patricia_merkle_tree::node_data::inner_node::EdgePathLength;
use crate::patricia_merkle_tree::types::NodeIndex;

#[derive(Debug, Error)]
pub enum PathToBottomError {
    #[error("Tried to remove {n_edges:?} edges from a {length:?} length path.")]
    RemoveEdgesError {
        length: EdgePathLength,
        n_edges: EdgePathLength,
    },
}

#[derive(Debug, Error)]
pub enum LeafError {
    #[error("Deleted leaf at index {0:?} appears in the updated skeleton tree.")]
    DeletedLeafInSkeleton(NodeIndex),
    #[error("Missing modification data at index {0:?}.")]
    MissingDataForUpdate(NodeIndex),
}

pub type LeafResult<T> = Result<T, LeafError>;
