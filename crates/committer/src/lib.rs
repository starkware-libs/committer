// TODO(Dori, 3/3/2024): Delete this dummy code.
pub fn dummy() -> u8 {
    7
}
#[cfg(test)]
pub mod test {
    const TREE_HEIGHT: u8 = 10;
    use rand::Rng;
    use std::time::Duration;
    use std::time::Instant;

    use crate::test_utils::{mean, random_felt, std_deviation};

    use super::dummy;
    use pathfinder_crypto::Felt;
    use pathfinder_crypto::MontFelt;
    use pretty_assertions::assert_eq;
    use starknet::core::types::FieldElement;
    use starknet_types_core::felt::Felt as StarknetFelt;
    use starknet_types_core::hash::Pedersen;
    use starknet_types_core::hash::StarkHash;

    use async_recursion::async_recursion;

    use rayon::Scope;

    #[test]
    fn test_dummy() {
        assert_eq!(dummy(), 7);
    }

    //run with `cargo test --release -- --nocapture bench`
    #[test]
    fn bench() {
        let num_iterations: usize = 10000;
        let num_repetitions: usize = 1;
        let mut time_pf: Vec<Duration> = Vec::new();
        let mut time_rs: Vec<Duration> = Vec::new();
        let mut time_sn: Vec<Duration> = Vec::new();

        for _i in 0..num_repetitions {
            let random_numbers: Vec<[u8; 32]> = (0..num_iterations)
                .map(|_| rand::thread_rng().gen())
                .collect();
            let random_mont_felts: Vec<MontFelt> = random_numbers
                .iter()
                .map(|x| MontFelt::from_be_bytes(*x))
                .collect();
            let random_felts: Vec<Felt> =
                random_mont_felts.iter().map(|x| Felt::from(*x)).collect();

            let random_field_elements: Vec<FieldElement> = random_felts
                .iter()
                .map(|x| {
                    FieldElement::from_bytes_be(x.as_be_bytes())
                        .expect("Overflow should not happen.")
                })
                .collect();
            let random_sn_felts: Vec<StarknetFelt> = random_numbers
                .iter()
                .map(StarknetFelt::from_bytes_be)
                .collect();

            let mut result_rs = random_field_elements[0];
            let mut result_pathfinder = random_felts[0];
            let mut result_pathfinder_poseidon = random_mont_felts[0];
            let mut result_sn = random_sn_felts[0];

            let now = Instant::now();
            // First code block to measure.
            for random_felt in random_felts.iter() {
                result_pathfinder =
                    pathfinder_crypto::hash::pedersen_hash(*random_felt, result_pathfinder);
            }
            let elapsed_pathfinder = now.elapsed();
            time_pf.push(elapsed_pathfinder);
            let now = Instant::now();
            // Second code block to measure.
            for random_field_element in random_field_elements.iter() {
                result_rs = starknet::core::crypto::pedersen_hash(random_field_element, &result_rs);
            }
            let elapsed_rs = now.elapsed();
            time_rs.push(elapsed_rs);
            let now = Instant::now();
            // Third code block to measure.
            for random_sn_felt in random_sn_felts.iter() {
                result_sn = Pedersen::hash(random_sn_felt, &result_sn);
            }
            let elapsed_sn = now.elapsed();
            time_sn.push(elapsed_sn);
            let now = Instant::now();
            // Fourth code block to measure.
            for random_mont_felt in random_mont_felts.iter() {
                result_pathfinder_poseidon = pathfinder_crypto::hash::poseidon_hash(
                    *random_mont_felt,
                    result_pathfinder_poseidon,
                );
            }
            let elapsed_pathfinder_poseidon = now.elapsed();
            // Print results.
            if num_repetitions == 1 {
                println!(
            "Time for {num_iterations} pederson hashes using pathfinder: {:?}, for a single hash: {:?}",
            elapsed_pathfinder,
            elapsed_pathfinder.checked_div(num_iterations.try_into().unwrap()),
        );
                println!(
                "Time for {num_iterations} pederson hashes using starknet-rs: {:?}, for a single hash: {:?}",
                elapsed_rs,
                elapsed_rs.checked_div(num_iterations.try_into().unwrap()),
            );
                println!(
            "Time for {num_iterations} pederson hashes using Starknet (LC): {:?}, for a single hash: {:?}",
            elapsed_sn,
            elapsed_sn.checked_div(num_iterations.try_into().unwrap()),
        );
                println!(
            "Time for {num_iterations} poseidon hashes using pathfinder: {:?}, for a single hash: {:?}",
            elapsed_pathfinder_poseidon,
            elapsed_pathfinder_poseidon.checked_div(num_iterations.try_into().unwrap()),
        );
            }
            // Sanity check.
            assert_eq!(
                &result_rs.to_bytes_be(),
                result_pathfinder.as_be_bytes(),
                "Results are not equal"
            );
            assert_eq!(
                result_rs.to_bytes_be(),
                result_sn.to_bytes_be(),
                "Results are not equal"
            );
        }
        // Print statistics.
        if num_repetitions > 1 {
            println!(
            "Average time for {num_iterations} hashes using pathfinder: {:?}, Std deviation: {:?}",
            mean(&time_pf),
            std_deviation(&time_pf),
        );
            println!(
                "Average time for {num_iterations} hashes using starknet-rs: {:?}, Std deviation: {:?}",
                mean(&time_rs),
                std_deviation(&time_rs),
            );
            println!(
            "Average time for {num_iterations} hashes using Starknet (LC): {:?}, Std deviation: {:?}",
            mean(&time_sn),
            std_deviation(&time_sn),
        );
        }
    }

    //Thread benchmarking

    pub struct SNTreeNode {
        left_child: Option<Box<SNTreeNode>>,
        right_child: Option<Box<SNTreeNode>>,
        _parent: Option<Box<SNTreeNode>>,
        hash_value: Option<Felt>,
        is_path: bool,
        _path_data: Option<[u128; 2]>,
        _is_root: bool,
        is_leaf: bool,
        // _leaf_data: Option<(ClassHash, ClassTreeNode)>,
    }

    pub fn create_dummy_tree(height: u8) -> Option<Box<SNTreeNode>> {
        let node = match height {
            TREE_HEIGHT => SNTreeNode {
                left_child: create_dummy_tree(height - 1),
                right_child: create_dummy_tree(height - 1),
                _parent: None,
                hash_value: None,
                is_path: false,
                _path_data: None,
                _is_root: true,
                is_leaf: false,
            },
            0 => SNTreeNode {
                left_child: None,
                right_child: None,
                _parent: None,
                hash_value: Some(random_felt()),
                is_path: false,
                _path_data: None,
                _is_root: false,
                is_leaf: true,
            },
            _ => SNTreeNode {
                left_child: create_dummy_tree(height - 1),
                right_child: create_dummy_tree(height - 1),
                _parent: None,
                hash_value: None,
                is_path: false,
                _path_data: None,
                _is_root: false,
                is_leaf: false,
            },
        };
        Some(Box::new(node))
    }

    pub fn clone_tree(node: &Option<Box<SNTreeNode>>) -> Option<Box<SNTreeNode>> {
        match node {
            Some(node) => {
                let mut new_node = SNTreeNode {
                    left_child: None,
                    right_child: None,
                    _parent: None,
                    hash_value: node.hash_value,
                    is_path: node.is_path,
                    _path_data: node._path_data,
                    _is_root: node._is_root,
                    is_leaf: node.is_leaf,
                };
                new_node.left_child = clone_tree(&node.left_child);
                new_node.right_child = clone_tree(&node.right_child);
                Some(Box::new(new_node))
            }
            None => None,
        }
    }

    pub fn count_inner_nodes(node: &Option<Box<SNTreeNode>>) -> u32 {
        match node {
            Some(node) => {
                let mut count = 0;
                if !node.is_leaf {
                    count += 1;
                    count += count_inner_nodes(&node.left_child);
                    count += count_inner_nodes(&node.right_child);
                }
                count
            }
            None => 0,
        }
    }

    pub async fn compute_val(node: SNTreeNode) -> Felt {
        match node.hash_value {
            Some(value) => value,
            None => algorithm_tokio(node).await,
        }
    }

    #[async_recursion]
    pub async fn algorithm_tokio(mut node: SNTreeNode) -> Felt {
        if node.is_leaf {
            //TODO: compute/return the leaf hash
            return node.hash_value.unwrap();
        }
        let left_child = node
            .left_child
            .expect("Not a leaf node, left child must exist");
        let right_child = node
            .right_child
            .expect("Not a leaf node, right child must exist");
        let left_value_future = tokio::spawn(compute_val(*left_child));
        let right_value_future = tokio::spawn(compute_val(*right_child));
        if !node.is_path {
            let (left_value, right_value) = (
                left_value_future.await.unwrap(),
                right_value_future.await.unwrap(),
            );
            node.hash_value = Some(pathfinder_crypto::hash::pedersen_hash(
                left_value,
                right_value,
            ));
            return node.hash_value.unwrap();
        } else {
            //TODO: compute/return the path hash
            todo!("Path hash computation")
        }
    }

    pub fn compute_val_rayon(node: SNTreeNode) -> Felt {
        match node.hash_value {
            Some(value) => value,
            None => algorithm_rayon(node),
        }
    }
    pub fn algorithm_rayon(mut node: SNTreeNode) -> Felt {
        if node.is_leaf {
            return node.hash_value.unwrap();
        }
        let left_child = node
            .left_child
            .expect("Not a leaf node, left child must exist");
        let right_child = node
            .right_child
            .expect("Not a leaf node, right child must exist");
        let mut left_value: Felt = Default::default();
        let mut right_value: Felt = Default::default();
        rayon::scope(|s: &Scope<'_>| {
            s.spawn(|_s| {
                left_value = compute_val_rayon(*left_child);
            });
            s.spawn(|_s| {
                right_value = compute_val_rayon(*right_child);
            });
        });
        if !node.is_path {
            node.hash_value = Some(pathfinder_crypto::hash::pedersen_hash(
                left_value,
                right_value,
            ));
            node.hash_value.unwrap()
        } else {
            //TODO: compute/return the path hash
            todo!("Path hash computation")
        }
    }

    pub fn algorithm_seq(mut node: SNTreeNode) -> Felt {
        if node.is_leaf {
            return node.hash_value.unwrap();
        }
        let left_child = node
            .left_child
            .expect("Not a leaf node, left child must exist");
        let right_child = node
            .right_child
            .expect("Not a leaf node, right child must exist");
        let left_value = algorithm_seq(*left_child);
        let right_value = algorithm_seq(*right_child);
        if !node.is_path {
            node.hash_value = Some(pathfinder_crypto::hash::pedersen_hash(
                left_value,
                right_value,
            ));
            node.hash_value.unwrap()
        } else {
            //TODO: compute/return the path hash
            todo!("Path hash computation")
        }
    }

    //run with `cargo test --release -- --nocapture bench_threading`
    #[tokio::test(flavor = "multi_thread")]
    async fn bench_threading() {
        let height = TREE_HEIGHT;
        let num_repetitions: usize = 3;
        let mut time_tokio: Vec<Duration> = Vec::new();
        let mut time_rayon: Vec<Duration> = Vec::new();
        let mut time_seq: Vec<Duration> = Vec::new();

        for _i in 0..num_repetitions {
            // fill the tree with data
            let now = Instant::now();
            let root = create_dummy_tree(height);
            let root_clone = clone_tree(&root);
            let root_clone_clone = clone_tree(&root_clone);
            let elapased_time = now.elapsed();
            println!("Tree creation time: {:?}", elapased_time);
            assert_eq!(count_inner_nodes(&root), 2_u32.pow(height.into()) - 1);

            let now = Instant::now();
            // Code block to measure.
            let result_tokio = algorithm_tokio(*root_clone.unwrap()).await;
            // End of code block to measure.
            let elapased_time_tokio = now.elapsed();
            time_tokio.push(elapased_time_tokio);
            // Print measurement.
            println!("Tokio time: {:?}", elapased_time_tokio);

            let now = Instant::now();
            // Code block to measure.
            let result_rayon = algorithm_rayon(*root_clone_clone.unwrap());
            // End of code block to measure.
            let elapased_time_rayon = now.elapsed();
            time_rayon.push(elapased_time_rayon);
            // Print measurement.
            println!("Rayon time: {:?}", elapased_time_rayon);

            let now = Instant::now();
            // Code block to measure.
            let result_seq = algorithm_seq(*root.unwrap());
            // End of code block to measure.
            let elapased_time_seq = now.elapsed();
            time_seq.push(elapased_time_seq);
            // Print measurement.
            println!("Sequential time: {:?}", elapased_time_seq);

            // Sanity check.
            assert_eq!(result_seq, result_tokio);
            assert_eq!(result_seq, result_rayon);
            println!("Sanity check passed!");
        }
        // Print statistics.
        println!(
            "Average time for {:?} hashes using tokio: {:?}, Std deviation: {:?}",
            2_u32.pow(height.into()) - 1,
            mean(&time_tokio),
            std_deviation(&time_tokio),
        );
        println!(
            "Average time for {:?} hashes using rayon: {:?}, Std deviation: {:?}",
            2_u32.pow(height.into()) - 1,
            mean(&time_rayon),
            std_deviation(&time_rayon),
        );
        println!(
            "Average time for {:?} hashes using sequential: {:?}, Std deviation: {:?}",
            2_u32.pow(height.into()) - 1,
            mean(&time_seq),
            std_deviation(&time_seq),
        );
    }
}

#[cfg(any(feature = "testing", test))]
pub mod test_utils;
