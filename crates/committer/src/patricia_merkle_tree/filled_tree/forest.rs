use crate::block_committer::input::ContractAddress;
use crate::hash::hash_trait::{HashFunction, HashOutput};
use crate::patricia_merkle_tree::filled_tree::node::{ClassHash, Nonce};
use crate::patricia_merkle_tree::filled_tree::tree::FilledTreeResult;
use crate::patricia_merkle_tree::filled_tree::tree::{FilledTree, FilledTreeImpl};
use crate::patricia_merkle_tree::node_data::leaf::{
    ContractState, LeafData, LeafDataImpl, LeafModifications,
};
use crate::patricia_merkle_tree::types::{NodeIndex, TreeHeight};
use crate::patricia_merkle_tree::updated_skeleton_tree::errors::UpdatedSkeletonTreeError;
use crate::patricia_merkle_tree::updated_skeleton_tree::hash_function::TreeHashFunction;
use crate::patricia_merkle_tree::updated_skeleton_tree::skeleton_forest::UpdatedSkeletonForestImpl;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::UpdatedSkeletonTree;
use crate::storage::storage_trait::Storage;

use futures::future::try_join_all;
use std::collections::HashMap;

pub trait FilledForest<L: LeafData> {
    #[allow(dead_code)]
    /// Serialize each tree and store it.
    fn write_to_storage(&self, storage: &mut impl Storage);
    #[allow(dead_code)]
    fn get_compiled_class_root_hash(&self) -> FilledTreeResult<HashOutput, L>;
    #[allow(dead_code)]
    fn get_contract_root_hash(&self) -> FilledTreeResult<HashOutput, L>;
}

pub struct FilledForestImpl {
    // TODO(Nimrod, 1/6/2024): Rename trees.
    storage_trees: HashMap<ContractAddress, FilledTreeImpl>,
    contract_tree: FilledTreeImpl,
    compiled_class_tree: FilledTreeImpl,
}

impl FilledForest<LeafDataImpl> for FilledForestImpl {
    #[allow(dead_code)]
    fn write_to_storage(&self, storage: &mut impl Storage) {
        // Serialize all trees to one hash map.
        let new_db_objects = self
            .storage_trees
            .values()
            .flat_map(|tree| tree.serialize().into_iter())
            .chain(self.contract_tree.serialize())
            .chain(self.compiled_class_tree.serialize())
            .collect();

        // Store the new hash map
        storage.mset(new_db_objects);
    }

    fn get_contract_root_hash(&self) -> FilledTreeResult<HashOutput, LeafDataImpl> {
        self.contract_tree.get_root_hash()
    }

    fn get_compiled_class_root_hash(&self) -> FilledTreeResult<HashOutput, LeafDataImpl> {
        self.compiled_class_tree.get_root_hash()
    }
}

impl FilledForestImpl {
    #[allow(dead_code)]
    pub(crate) async fn create<
        T: UpdatedSkeletonTree,
        H: HashFunction,
        TH: TreeHashFunction<LeafDataImpl, H>,
    >(
        updated_forest: &UpdatedSkeletonForestImpl<T>,
        storage_updates: &HashMap<ContractAddress, LeafModifications<LeafDataImpl>>,
        classes_updates: &LeafModifications<LeafDataImpl>,
        current_contracts_trie_leaves: &HashMap<ContractAddress, ContractState>,
        address_to_class_hash: &HashMap<ContractAddress, ClassHash>,
        address_to_nonce: &HashMap<ContractAddress, Nonce>,
        tree_heights: TreeHeight,
    ) -> FilledTreeResult<Self, LeafDataImpl> {
        let classes_trie =
            FilledTreeImpl::create::<H, TH>(&updated_forest.classes_trie, classes_updates).await?;

        let mut contracts_trie_modifications = HashMap::new();
        let mut filled_storage_tries = HashMap::new();
        let mut tasks = vec![];

        for (address, inner_updates) in storage_updates {
            let updated_storage_trie = updated_forest
                .storage_tries
                .get(address)
                .ok_or(UpdatedSkeletonTreeError::LowerTreeCommitmentError(*address))?;

            let old_contract_state = current_contracts_trie_leaves
                .get(address)
                // TODO(Nimrod, 1/6/2024): Add another error variant for that case.
                .ok_or(UpdatedSkeletonTreeError::LowerTreeCommitmentError(*address))?;
            tasks.push(Self::new_contract_state::<T, H, TH>(
                *address,
                address_to_nonce
                    .get(address)
                    .unwrap_or(&old_contract_state.nonce),
                address_to_class_hash
                    .get(address)
                    .unwrap_or(&old_contract_state.class_hash),
                updated_storage_trie,
                inner_updates,
            ));
        }
        for (address, new_contract_state, filled_storage_trie) in try_join_all(tasks).await? {
            contracts_trie_modifications.insert(
                NodeIndex::from_contract_address(&address, &tree_heights),
                LeafDataImpl::ContractState(new_contract_state),
            );
            filled_storage_tries.insert(address, filled_storage_trie);
        }
        let contracts_trie = FilledTreeImpl::create::<H, TH>(
            &updated_forest.contracts_trie,
            &contracts_trie_modifications,
        )
        .await?;

        Ok(Self {
            storage_trees: filled_storage_tries,
            contract_tree: contracts_trie,
            compiled_class_tree: classes_trie,
        })
    }

    async fn new_contract_state<
        T: UpdatedSkeletonTree,
        H: HashFunction,
        TH: TreeHashFunction<LeafDataImpl, H>,
    >(
        contract_address: ContractAddress,
        new_nonce: &Nonce,
        new_class_hash: &ClassHash,
        updated_storage_trie: &T,
        inner_updates: &LeafModifications<LeafDataImpl>,
    ) -> FilledTreeResult<(ContractAddress, ContractState, FilledTreeImpl), LeafDataImpl> {
        let filled_storage_trie =
            FilledTreeImpl::create::<H, TH>(updated_storage_trie, inner_updates).await?;
        let new_root_hash = filled_storage_trie.get_root_hash()?;
        Ok((
            contract_address,
            ContractState {
                nonce: *new_nonce,
                storage_root_hash: new_root_hash,
                class_hash: *new_class_hash,
            },
            filled_storage_trie,
        ))
    }
}
