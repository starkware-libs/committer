// TODO(Dori, 3/3/2024): Delete this dummy code.
pub fn dummy() -> u8 {
    7
}
#[cfg(test)]
pub mod test {
    use rand::Rng;
    use std::time::Duration;
    use std::time::Instant;

    use super::dummy;
    use pathfinder_crypto::Felt;
    use pathfinder_crypto::MontFelt;
    use pretty_assertions::assert_eq;
    use starknet::core::types::FieldElement;
    use starknet_types_core::felt::Felt as StarknetFelt;
    use starknet_types_core::hash::Pedersen;
    use starknet_types_core::hash::StarkHash;

    #[test]
    fn test_dummy() {
        assert_eq!(dummy(), 7);
    }

    fn mean(data: &[Duration]) -> Duration {
        data.iter()
            .sum::<Duration>()
            .checked_div(data.len().try_into().unwrap())
            .unwrap()
    }
    #[allow(clippy::as_conversions)]
    fn std_deviation(data: &[Duration]) -> Duration {
        let mean = mean(data).as_secs_f32();
        let mut variance = data
            .iter()
            .map(|x| {
                let diff = (*x).as_secs_f32() - mean;
                diff * diff
            })
            .sum::<f32>();
        variance /= data.len() as f32;
        Duration::from_secs_f32(variance.sqrt())
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

    //run with `cargo test --release -- --nocapture bench_threading`
    #[test]
    fn bench_threading() {}
}
