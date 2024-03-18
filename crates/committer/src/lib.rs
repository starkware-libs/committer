// TODO(Dori, 3/3/2024): Delete this dummy code.
pub fn dummy() -> u8 {
    7
}
#[cfg(test)]
pub mod test {
    use std::time::Instant;

    use super::dummy;
    use pathfinder_crypto::Felt;
    // use pathfinder_crypto::hash::pedersen_hash;
    use pretty_assertions::assert_eq;
    use starknet::core::types::FieldElement;

    #[test]
    fn test_dummy() {
        assert_eq!(dummy(), 7);
    }

    //run with `cargo test --release -- --nocapture bench`
    #[test]
    fn bench() {
        let num_iterations: u32 = 10000;
        let mut result_lc = FieldElement::ZERO;
        let mut result_pathfinder = Felt::from_u64(0);
        let one_field: FieldElement = FieldElement::ONE;
        let one_felt: Felt = Felt::from_u64(1);

        let now = Instant::now();
        // First code block to measure.
        for _ in 0..num_iterations {
            result_pathfinder = pathfinder_crypto::hash::pedersen_hash(one_felt, result_pathfinder);
        }
        let elapsed_pathfinder = now.elapsed();
        // Second code block to measure.
        for _ in 0..num_iterations {
            result_lc = starknet::core::crypto::pedersen_hash(&one_field, &result_lc);
        }
        let elapsed_lc = now.elapsed() - elapsed_pathfinder;
        // Print results.
        println!(
            "Time for {num_iterations} pederson hashes using pathfinder: {:?}, for a single hash: {:?}",
            elapsed_pathfinder,
            elapsed_pathfinder.checked_div(num_iterations)
        );
        println!(
            "Time for {num_iterations} pederson hashes using LC: {:?}, for a single hash: {:?}",
            elapsed_lc,
            elapsed_lc.checked_div(num_iterations),
        );
        // Sanity check - assert results are equal.
        assert_eq!(
            &result_lc.to_bytes_be(),
            result_pathfinder.as_be_bytes(),
            "Results are not equal"
        );
    }
}
