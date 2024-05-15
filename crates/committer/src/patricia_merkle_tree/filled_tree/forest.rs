use crate::patricia_merkle_tree::filled_tree::tree::FilledTree;
use crate::patricia_merkle_tree::node_data::leaf::LeafData;
use crate::storage::map_storage::MapStorage;
use crate::storage::storage_trait::Storage;
use std::collections::{HashMap, HashSet};

pub(crate) trait FilledForest<L: LeafData> {
    #[allow(dead_code)]
    /// Write the forest to storage. Serialize each tree and store it in a storage object.
    fn write_to_storage(&self);
}

pub(crate) struct FilledForestImpl<L: LeafData> {
    forest: HashSet<Box<dyn FilledTree<L>>>,
}

impl<L: LeafData> FilledForest<L> for FilledForestImpl<L> {
    #[allow(dead_code)]
    fn write_to_storage(&self) {
        // Create a new storage object
        let mut fact_storage = MapStorage {
            storage: HashMap::new(),
        };

        // Serialize each tree and store it in the storage object
        for tree in self.forest.iter() {
            fact_storage.mset(tree.serialize())
        }

        // Serialize the storage to a JSON string and Print it for Python.
        match serde_json::to_string(&fact_storage) {
            Ok(json_string) => print!("{}", json_string),
            Err(e) => eprintln!("Failed to serialize fact_storage: {}", e),
        }
    }
}
