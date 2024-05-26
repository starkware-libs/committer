use std::{
    fs, thread,
    time::{Duration, Instant},
};
#[allow(unused_variables)]
fn main() {
    // 0. Set up parameters

    let tree_height = 10;
    let full_binary = true;

    // 1. Create random-like tree

    let now = Instant::now();
    // Code block to measure.
    thread::sleep(Duration::from_secs(rand::random::<u64>() % 3));
    // End of code block to measure.
    let elapased_time_tree_creation = now.elapsed();

    // 2. Run hash function on tree with time measurement
    let now = Instant::now();
    // Code block to measure.
    thread::sleep(Duration::from_secs(rand::random::<u64>() % 3));
    // End of code block to measure.
    let elapased_time_main_algorithm = now.elapsed();

    // 3. Compare the root hash with expected root hash.

    // 4. Print benchmarking results to file
    // TODO (Aner, 26/05/24): Add more information to the benchmark, e.g., number of leaves, type of tree, number of nodes, etc.
    let benchmark_results = format!(
        "Tree height: {:?}\nFull binary: {:?}\nTree creation time: {:?}\nMain algorithm time: {:?}\n",
        tree_height,
        full_binary,
        elapased_time_tree_creation,
        elapased_time_main_algorithm
    );
    fs::write("benchmark.txt", benchmark_results).expect("Unable to write file");
}
