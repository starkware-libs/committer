use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::filled_tree::errors::FilledTreeError;
use crate::patricia_merkle_tree::filled_tree::tree::FilledTree;
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::storage::serde_trait::DBObject;
use crate::storage::storage_trait::Storage;
use std::collections::HashMap;
use std::marker::PhantomData;

pub(crate) trait FilledForest<L: LeafData, S: Storage> {
    #[allow(dead_code)]
    /// Serialize each tree and store it.
    fn write_to_storage(&self, storage: &mut S);
    #[allow(dead_code)]
    fn get_compiled_class_root_hash(&self) -> Result<HashOutput, FilledTreeError<L>>;
    #[allow(dead_code)]
    fn get_contract_root_hash(&self) -> Result<HashOutput, FilledTreeError<L>>;
}

pub(crate) struct FilledForestImpl<L: LeafData + DBObject, T: FilledTree<L>> {
    storage_trees: HashMap<NodeIndex, T>,
    contract_tree: T,
    compiled_class_tree: T,
    phantom: PhantomData<L>,
}

impl<L: LeafData + DBObject, T: FilledTree<L>, S: Storage> FilledForest<L, S>
    for FilledForestImpl<L, T>
{
    #[allow(dead_code)]
    fn write_to_storage(&self, storage: &mut S) {
        // Serialize each storage tree and store it in the storage object
        for tree in self.storage_trees.values() {
            storage.mset(tree.serialize())
        }

        // Store the compiled class tree.
        storage.mset(self.compiled_class_tree.serialize());

        // Store the contract tree.
        storage.mset(self.contract_tree.serialize());
    }

    fn get_contract_root_hash(&self) -> Result<HashOutput, FilledTreeError<L>> {
        self.contract_tree.get_root_hash()
    }

    fn get_compiled_class_root_hash(&self) -> Result<HashOutput, FilledTreeError<L>> {
        self.compiled_class_tree.get_root_hash()
    }
}
