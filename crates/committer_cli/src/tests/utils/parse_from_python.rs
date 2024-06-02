use committer::felt::Felt;
use committer::hash::hash_trait::HashOutput;
use committer::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use committer::patricia_merkle_tree::node_data::leaf::LeafModifications;
use committer::patricia_merkle_tree::types::NodeIndex;
use committer::patricia_merkle_tree::types::TreeHeight;
use committer::storage::map_storage::MapStorage;
use committer::storage::storage_trait::StorageKey;
use committer::storage::storage_trait::StorageValue;
use ethnum::U256;
use std::collections::HashMap;

#[allow(clippy::unwrap_used)]
/// Parse input for single tree flow test.
pub fn parse_input_single_tree_flow_test(
    input: HashMap<String, String>,
) -> (
    TreeHeight,
    LeafModifications<LeafDataImpl>,
    MapStorage,
    HashOutput, //root_hash
) {
    // Fetch tree height.
    let tree_height_str = input.get("tree_height").unwrap();
    let tree_height = TreeHeight(tree_height_str.parse::<u8>().unwrap());

    // Fetch leaf_modifications.
    let leaf_modifications_json = input.get("leaf_modifications").unwrap();
    let leaf_modifications_map =
        serde_json::from_str::<HashMap<u128, &str>>(leaf_modifications_json).unwrap();
    let leaf_modifications = leaf_modifications_map
        .iter()
        .map(|(k, v)| {
            (
                NodeIndex::new(U256::from(*k)),
                LeafDataImpl::StorageValue(Felt::from_hex(v).unwrap()),
            )
        })
        .collect();

    // Fetch storage.
    let storage: HashMap<StorageKey, StorageValue> =
        serde_json::from_str::<HashMap<&str, &str>>(input.get("storage").unwrap())
            .expect("serde json failed to parse storage")
            .into_iter()
            .map(|(k, v)| {
                (
                    StorageKey(Felt::from_hex(k).unwrap().to_bytes_be().to_vec()),
                    StorageValue(Felt::from_hex(v).unwrap().to_bytes_be().to_vec()),
                )
            })
            .collect();

    let map_storage = MapStorage::from(storage);

    // Fetch root_hash.
    let root_hash = HashOutput(Felt::from_hex(input.get("root_hash").unwrap()).unwrap());

    (tree_height, leaf_modifications, map_storage, root_hash)
}
