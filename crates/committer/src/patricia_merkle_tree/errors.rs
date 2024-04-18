// TODO(Amos, 01/04/2024): Add error types.

use crate::patricia_merkle_tree::types::NodeIndex;

use crate::patricia_merkle_tree::filled_node::FilledNode;

use super::types::LeafDataTrait;

#[derive(Debug)]
pub(crate) enum OriginalSkeletonTreeError {}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum UpdatedSkeletonTreeError<L: LeafDataTrait> {
    MissingNode(NodeIndex),
    DoubleUpdate {
        index: NodeIndex,
        existing_value: Box<FilledNode<L>>,
    },
    PoisonedLock(String),
    NonDroppedPointer(String),
}

#[derive(thiserror::Error, Debug, derive_more::Display)]
pub(crate) enum FilledTreeError {
    MissingRoot,
    SerializeError(#[from] serde_json::Error),
}
