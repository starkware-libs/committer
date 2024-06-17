use committer::patricia_merkle_tree::external_test_utils::single_tree_flow_test;
use committer_cli::tests::utils::parse_from_python::parse_input_single_storage_tree_flow_test;
use criterion::{criterion_group, criterion_main, Criterion};

const CONCURRENCY_MODE: bool = true;

#[allow(clippy::unwrap_used)]
pub fn single_tree_flow_benchmark(criterion: &mut Criterion) {
    let (leaf_modifications, storage, root_hash) = parse_input_single_storage_tree_flow_test(
        &serde_json::from_str(&std::fs::read_to_string("inputs.json").unwrap()).unwrap(),
    );

    let runtime = match CONCURRENCY_MODE {
        true => tokio::runtime::Builder::new_multi_thread().build().unwrap(),
        false => tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap(),
    };

    criterion.bench_function("single_tree_flow_test", |benchmark| {
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
