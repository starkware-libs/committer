use crate::{
    block_committer::input::{ContractAddress, StarknetStorageValue},
    starknet_patricia_merkle_tree::{node::CompiledClassHash, starknet_leaf::leaf::ContractState},
};
use committer::patricia_merkle_tree::filled_tree::{errors::FilledTreeError, tree::FilledTreeImpl};
use std::collections::HashMap;

pub type StorageTrie = FilledTreeImpl<StarknetStorageValue>;
pub type ClassesTrie = FilledTreeImpl<CompiledClassHash>;
pub type ContractsTrie = FilledTreeImpl<ContractState>;
pub type StorageTrieMap = HashMap<ContractAddress, StorageTrie>;

pub type StorageTrieError = FilledTreeError<StarknetStorageValue>;
pub type ClassesTrieError = FilledTreeError<CompiledClassHash>;
pub type ContractsTrieError = FilledTreeError<ContractState>;

#[cfg(test)]
pub mod types_test {
    #[rstest]
    fn test_cast_to_node_index(
        #[values(0, 15, 0xDEADBEEF)] leaf_index: u128,
        #[values(true, false)] from_contract_address: bool,
    ) {
        let expected_node_index = NodeIndex::FIRST_LEAF + leaf_index;
        let actual = if from_contract_address {
            NodeIndex::from_contract_address(&ContractAddress(Felt::from(leaf_index)))
        } else {
            NodeIndex::from_starknet_storage_key(&StarknetStorageKey(Felt::from(leaf_index)))
        };
        assert_eq!(actual, expected_node_index);
    }
}
