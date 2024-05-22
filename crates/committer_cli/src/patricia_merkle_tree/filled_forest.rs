use crate::patricia_merkle_tree::errors::FilledForestError;
use committer::patricia_merkle_tree::filled_tree::forest::{FilledForest, FilledForestImpl};
use committer::patricia_merkle_tree::filled_tree::tree::FilledTree;
use committer::patricia_merkle_tree::node_data::leaf::LeafData;
use committer::storage::map_storage::MapStorage;
use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) trait ForestToPython<L: LeafData, T: FilledTree<L>> {
    /// Print all the new relevant data to stdout in the format expected by the Python.
    /// Return Ok if succeeded.
    fn forest_to_python(&self) -> Result<(), FilledForestError<L>>;
}

impl<L: LeafData, T: FilledTree<L>> ForestToPython<L, T> for FilledForestImpl<L, T> {
    fn forest_to_python(&self) -> Result<(), FilledForestError<L>> {
        let mut storage: MapStorage = MapStorage {
            storage: HashMap::new(),
        };
        self.write_to_storage(&mut storage);

        // Output the new fact storage.
        println!("{}", serde_json::to_string(&storage)?);

        // Output the new contract storage root.
        let contract_storage_root_hash = self.get_contract_root_hash()?.0;
        println!("{}", contract_storage_root_hash.to_string()?);

        // Output the new compiled class storage root.
        let compiled_class_root_hash = self.get_compiled_class_root_hash()?.0;
        println!("{}", compiled_class_root_hash.to_string()?);

        Ok(())
    }
}
