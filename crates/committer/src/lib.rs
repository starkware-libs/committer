// TODO(Dori, 3/3/2024): Delete this dummy code.
pub fn dummy() -> u8 {
    7
}
#[cfg(test)]
pub mod test {
    use rand::Rng;
    use std::time::Instant;

    use super::dummy;
    use pathfinder_crypto::Felt;
    use pathfinder_crypto::MontFelt;
    use pretty_assertions::assert_eq;
    use starknet::core::types::FieldElement;

    #[test]
    fn test_dummy() {
        assert_eq!(dummy(), 7);
    }

    //run with `cargo test --release -- --nocapture bench`
    #[test]
    fn bench() {
        let num_iterations: usize = 10000;

        let random_numbers: Vec<[u8; 32]> = (0..num_iterations)
            .map(|_| rand::thread_rng().gen())
            .collect();
        let random_mont_felts: Vec<MontFelt> = random_numbers
            .iter()
            .map(|x| MontFelt::from_be_bytes(*x))
            .collect();
        let random_felts: Vec<Felt> = random_mont_felts.iter().map(|x| Felt::from(*x)).collect();

        let random_field_elements: Vec<FieldElement> = random_felts
            .iter()
            .map(|x| {
                FieldElement::from_bytes_be(x.as_be_bytes()).expect("Overflow should not happen.")
            })
            .collect();

        let mut result_lc = random_field_elements[0];
        let mut result_pathfinder = random_felts[0];
        let mut result_pathfinder_poseidon = random_mont_felts[0];

        let now = Instant::now();
        // First code block to measure.
        for random_felt in random_felts.iter() {
            result_pathfinder =
                pathfinder_crypto::hash::pedersen_hash(*random_felt, result_pathfinder);
        }
        let elapsed_pathfinder = now.elapsed();
        let now = Instant::now();
        // Second code block to measure.
        for random_field_element in random_field_elements.iter() {
            result_lc = starknet::core::crypto::pedersen_hash(random_field_element, &result_lc);
        }
        let elapsed_lc = now.elapsed();
        let now = Instant::now();
        // Third code block to measure.
        for random_mont_felt in random_mont_felts.iter() {
            result_pathfinder_poseidon = pathfinder_crypto::hash::poseidon_hash(
                *random_mont_felt,
                result_pathfinder_poseidon,
            );
        }
        let elapsed_pathfinder_poseidon = now.elapsed();
        // Print results.
        println!(
            "Time for {num_iterations} pederson hashes using pathfinder: {:?}, for a single hash: {:?}",
            elapsed_pathfinder,
            elapsed_pathfinder.checked_div(num_iterations.try_into().unwrap()),
        );
        println!(
            "Time for {num_iterations} pederson hashes using LC: {:?}, for a single hash: {:?}",
            elapsed_lc,
            elapsed_lc.checked_div(num_iterations.try_into().unwrap()),
        );
        println!(
            "Time for {num_iterations} poseidon hashes using pathfinder: {:?}, for a single hash: {:?}",
            elapsed_pathfinder_poseidon,
            elapsed_pathfinder_poseidon.checked_div(num_iterations.try_into().unwrap()),
        );
        assert_eq!(
            &result_lc.to_bytes_be(),
            result_pathfinder.as_be_bytes(),
            "Results are not equal"
        );
    }
}
