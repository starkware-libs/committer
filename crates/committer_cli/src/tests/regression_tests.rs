use std::collections::HashMap;

use committer::patricia_merkle_tree::external_test_utils::single_tree_flow_test;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

use crate::{
    commands::commit, parse_input::read::parse_input,
    tests::utils::parse_from_python::TreeFlowInput,
};

use super::utils::parse_from_python::parse_input_single_storage_tree_flow_test;

//TODO(Aner, 20/06/2024): these tests needs to be fixed to be run correctly in the CI:
//1. Fix the test to measure cpu_time and not wall_time.
//2. Fix the max time threshold to be the expected time for the benchmark test.
const MAX_TIME_FOR_SINGLE_TREE_BECHMARK_TEST: f64 = 5.0;
const MAX_TIME_FOR_COMMITTER_FLOW_BECHMARK_TEST: f64 = 5.0;
const SINGLE_TREE_FLOW_INPUT: &str = include_str!("../../benches/tree_flow_inputs.json");
const FLOW_TEST_INPUT: &str = include_str!("../../benches/committer_flow_inputs.json");
const OUTPUT_PATH: &str = "benchmark_output.txt";

struct FactMap(Map<String, Value>);

impl<'de> Deserialize<'de> for FactMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(
            serde_json::from_str(&String::deserialize(deserializer)?).unwrap(),
        ))
    }
}

#[derive(Deserialize)]
struct CommitterRegressionInput {
    committer_input: String,
    contract_states_root: String,
    contract_classes_root: String,
    expected_facts: FactMap,
}

#[derive(Deserialize)]
struct TreeRegressionOutput {
    root_hash: Value,
    storage_changes: Value,
}

#[derive(Deserialize)]
struct StorageObject {
    storage: Value,
}

#[derive(Deserialize)]
struct CommitterRegressionOutput {
    contract_storage_root_hash: Value,
    compiled_class_root_hash: Value,
    storage: StorageObject,
}

struct TreeRegressionInput {
    tree_flow_input: TreeFlowInput,
    expected_hash: String,
    expected_storage_changes: Map<String, Value>,
}

// TODO(Aner, 9/8/24): remove this impl and use the Deserialize derive, by changing the input format.
impl<'de> Deserialize<'de> for TreeRegressionInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = HashMap::deserialize(deserializer)?;
        Ok(Self {
            tree_flow_input: parse_input_single_storage_tree_flow_test(&map),
            expected_hash: map.get("expected_hash").unwrap().to_string(),
            expected_storage_changes: serde_json::from_str(
                map.get("expected_storage_changes").unwrap(),
            )
            .unwrap(),
        })
    }
}

#[ignore = "To avoid running the benchmark test in Coverage or without the --release flag."]
#[tokio::test(flavor = "multi_thread")]
pub async fn test_regression_single_tree() {
    let TreeRegressionInput {
        tree_flow_input:
            TreeFlowInput {
                leaf_modifications,
                storage,
                root_hash,
            },
        expected_hash,
        expected_storage_changes,
    } = serde_json::from_str(SINGLE_TREE_FLOW_INPUT).unwrap();

    let start = std::time::Instant::now();
    // Benchmark the single tree flow test.
    let output = single_tree_flow_test(leaf_modifications, storage, root_hash).await;
    let execution_time = std::time::Instant::now() - start;

    // Assert correctness of the output of the single tree flow test.
    let TreeRegressionOutput {
        root_hash,
        storage_changes: Value::Object(actual_storage_changes),
    } = serde_json::from_str(&output).unwrap()
    else {
        panic!("Expected storage changes object to be an object.");
    };
    assert_eq!(root_hash, expected_hash);

    assert_eq!(actual_storage_changes, expected_storage_changes);

    // 4. Assert the execution time does not exceed the threshold.
    assert!(execution_time.as_secs_f64() < MAX_TIME_FOR_SINGLE_TREE_BECHMARK_TEST);
}

#[ignore = "To avoid running the benchmark test in Coverage or without the --release flag."]
#[tokio::test(flavor = "multi_thread")]
pub async fn test_regression_committer_flow() {
    let CommitterRegressionInput {
        committer_input,
        contract_states_root: expected_contract_states_root,
        contract_classes_root: expected_contract_classes_root,
        expected_facts,
    } = serde_json::from_str(FLOW_TEST_INPUT).unwrap();

    let start = std::time::Instant::now();
    // Benchmark the committer flow test.
    // TODO(Aner, 9/7/2024): refactor committer_input to be a struct.
    let committer_input = parse_input(&committer_input).expect("Failed to parse the given input.");
    commit(committer_input, OUTPUT_PATH.to_owned()).await;
    let execution_time = std::time::Instant::now() - start;

    // Assert correctness of the output of the committer flow test.
    let CommitterRegressionOutput {
        contract_storage_root_hash,
        compiled_class_root_hash,
        storage: StorageObject {
            storage: Value::Object(storage_changes),
        },
    } = serde_json::from_str(&std::fs::read_to_string(OUTPUT_PATH).unwrap()).unwrap()
    else {
        panic!("Expected the storage to be an object.");
    };

    assert_eq!(contract_storage_root_hash, expected_contract_states_root);
    assert_eq!(compiled_class_root_hash, expected_contract_classes_root);
    assert_eq!(storage_changes, expected_facts.0);

    // Assert the execution time does not exceed the threshold.
    assert!(execution_time.as_secs_f64() < MAX_TIME_FOR_COMMITTER_FLOW_BECHMARK_TEST);
}
