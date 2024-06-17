pub(crate) fn add_unique<K, V>(
    map: &mut HashMap<K, V>,
    map_name: &str,
    key: K,
    value: V,
) -> Result<(), DeserializationError>
where
    K: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
{
    if map.contains_key(&key) {
        return Err(DeserializationError::KeyDuplicate(format!(
            "{map_name}: {key:?}"
        )));
    }
    map.insert(key, value);
    Ok(())
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawStorageEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

use committer::felt::Felt;
use committer::hash::hash_trait::HashOutput;
use committer::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use committer::patricia_merkle_tree::node_data::leaf::LeafModifications;
use committer::patricia_merkle_tree::types::NodeIndex;
use committer::storage::errors::DeserializationError;
use committer::storage::map_storage::MapStorage;
use committer::storage::storage_trait::StorageKey;
use committer::storage::storage_trait::StorageValue;
use ethnum::U256;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(clippy::unwrap_used)]
/// Parse input for single storage tree flow test.
/// Returns the leaf modifications, fetched nodes (in storage) and the root hash.
pub fn parse_input_single_storage_tree_flow_test(
    input: HashMap<String, String>,
) -> (LeafModifications<LeafDataImpl>, MapStorage, HashOutput) {
    // Fetch leaf_modifications.
    let leaf_modifications_json = input.get("leaf_modifications").unwrap();
    let leaf_modifications_map =
        serde_json::from_str::<HashMap<&str, &str>>(leaf_modifications_json).unwrap();
    let leaf_modifications = leaf_modifications_map
        .iter()
        .map(|(k, v)| {
            (
                NodeIndex::new(U256::from_str_hex(k).unwrap()),
                LeafDataImpl::StorageValue(Felt::from_hex(v).unwrap()),
            )
        })
        .collect();

    // Fetch storage.
    let raw_storage =
        serde_json::from_str::<Vec<RawStorageEntry>>(input.get("storage").unwrap()).unwrap();

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
    let root_hash = HashOutput(Felt::from_hex(input.get("root_hash").unwrap()).unwrap());

    (leaf_modifications, map_storage, root_hash)
}

use committer::patricia_merkle_tree::external_test_utils::single_tree_flow_test;
use criterion::{criterion_group, criterion_main, Criterion};

#[allow(clippy::unwrap_used)]
pub fn single_tree_flow_benchmark(c: &mut Criterion) {
    let concurrency_mode = true;

    let input: std::collections::HashMap<String, String> =
        serde_json::from_str(&std::fs::read_to_string("inputs.json").unwrap()).unwrap();

    let (leaf_modifications, storage, root_hash) = parse_input_single_storage_tree_flow_test(input);

    let runtime = match concurrency_mode {
        true => tokio::runtime::Builder::new_multi_thread().build().unwrap(),
        false => tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap(),
    };

    c.bench_function("single_tree_flow_test", |benchmark| {
        benchmark.iter(|| {
            runtime.block_on(single_tree_flow_test(
                leaf_modifications.clone(),
                storage.clone(),
                root_hash,
            ));
        })
    });
}

criterion_group!(benches, single_tree_flow_benchmark);
criterion_main!(benches);
