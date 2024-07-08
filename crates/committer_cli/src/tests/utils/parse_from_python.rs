use crate::parse_input::cast::add_unique;
use crate::parse_input::raw_input::RawStorageEntry;
use committer::block_committer::input::StarknetStorageValue;
use committer::felt::Felt;
use committer::hash::hash_trait::HashOutput;
use committer::patricia_merkle_tree::node_data::leaf::LeafModifications;
use committer::patricia_merkle_tree::types::NodeIndex;
use committer::storage::map_storage::MapStorage;
use committer::storage::storage_trait::StorageKey;
use committer::storage::storage_trait::StorageValue;
use ethnum::U256;
use std::collections::HashMap;

#[allow(clippy::unwrap_used)]
pub fn deserialize_input_single_storage_tree_flow_test(
    input: String,
) -> (
    LeafModifications<StarknetStorageValue>,
    MapStorage,
    HashOutput,
) {
    parse_input_single_storage_tree_flow_test_from_map(serde_json::from_str(&input).unwrap())
}
#[allow(clippy::unwrap_used)]
pub fn parse_input_single_storage_tree_flow_test_from_map(
    input: HashMap<String, String>,
) -> (
    LeafModifications<StarknetStorageValue>,
    MapStorage,
    HashOutput,
) {
    let leaf_modifications_json = input.get("leaf_modifications").unwrap();
    let storage_json = input.get("storage").unwrap();
    let root_hash = input.get("root_hash").unwrap();

    parse_input_single_storage_tree_flow_test(
        leaf_modifications_json.to_string(),
        storage_json.to_string(),
        root_hash.to_string(),
    )
}
#[allow(clippy::unwrap_used)]
/// Parse input for single storage tree flow test.
/// Returns the leaf modifications, fetched nodes (in storage) and the root hash.
pub fn parse_input_single_storage_tree_flow_test(
    leaf_modifications_json: String,
    storage_json: String,
    root_hash: String,
) -> (
    LeafModifications<StarknetStorageValue>,
    MapStorage,
    HashOutput,
) {
    // Fetch leaf_modifications.
    // let leaf_modifications_json = input.get("leaf_modifications").unwrap();
    let leaf_modifications_map =
        serde_json::from_str::<HashMap<&str, &str>>(&leaf_modifications_json).unwrap();
    let leaf_modifications = leaf_modifications_map
        .iter()
        .map(|(k, v)| {
            (
                NodeIndex::new(U256::from_str_hex(k).unwrap()),
                StarknetStorageValue(Felt::from_hex(v).unwrap()),
            )
        })
        .collect();

    // Fetch storage.
    let raw_storage = serde_json::from_str::<Vec<RawStorageEntry>>(&storage_json).unwrap();

    let mut storage = HashMap::new();
    for entry in raw_storage {
        add_unique(
            &mut storage,
            "storage",
            StorageKey(entry.key),
            StorageValue(entry.value),
        )
        .unwrap();
    }

    let map_storage = MapStorage { storage };

    // Fetch root_hash.
    let root_hash = HashOutput(Felt::from_hex(&root_hash).unwrap());

    (leaf_modifications, map_storage, root_hash)
}
