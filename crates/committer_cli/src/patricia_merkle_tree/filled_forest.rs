use crate::patricia_merkle_tree::errors::FilledForestError;
use committer::patricia_merkle_tree::filled_tree::forest::{FilledForestImpl, FilledForest};
use committer::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use committer::storage::map_storage::MapStorage;
use std::collections::HashMap;

#[allow(dead_code)]
pub(crate) struct SerializedForest (FilledForestImpl);


impl SerializedForest{
    #[allow(dead_code)]
    fn forest_to_python(&self) -> Result<(), FilledForestError<LeafDataImpl>> {
        let mut storage: MapStorage = MapStorage {
            storage: HashMap::new(),
        };
        self.0.write_to_storage(&mut storage);

        // Output the new fact storage.
        println!("{}", serde_json::to_string(&storage)?);

        // Output the new contract storage root.
        let contract_storage_root_hash = self.0.get_contract_root_hash()?.0;
        println!("{}", contract_storage_root_hash.to_string()?);

        // Output the new compiled class storage root.
        let compiled_class_root_hash = self.0.get_compiled_class_root_hash()?.0;
        println!("{}", compiled_class_root_hash.to_string()?);

        Ok(())
    }
}
