use committer::block_committer::input::StarknetStorageValue;
use committer::patricia_merkle_tree::filled_tree::errors::FilledTreeError;
use committer::patricia_merkle_tree::filled_tree::node::CompiledClassHash;
use committer::patricia_merkle_tree::node_data::leaf::ContractState;
use std::fmt::Debug;

#[derive(thiserror::Error, Debug)]
pub(crate) enum FilledForestError {
    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),
    #[error(transparent)]
    MissingStorageRoot(#[from] FilledTreeError<StarknetStorageValue>),
    #[error(transparent)]
    MissingCompiledClassRoot(#[from] FilledTreeError<CompiledClassHash>),
    #[error(transparent)]
    MissingContractStateRoot(#[from] FilledTreeError<ContractState>),
}
