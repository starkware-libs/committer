use std::env;
use std::path::Path;
// use std::process::Output;
use std::time::Instant;

use pathfinder_crypto::Felt;
use pathfinder_crypto::MontFelt;
use starknet::core::types::FieldElement;
use starknet_types_core::felt::Felt as StarknetFelt;
use starknet_types_core::hash::Pedersen;
use starknet_types_core::hash::StarkHash;

const PATHFINDER_PEDERSEN: &str = "pathfinder_pedersen";
const LC_PEDERSEN: &str = "lc_pedersen";
const STARKNET_PEDERSEN: &str = "sn_pedersen";
const PATHFINDER_POSEIDON: &str = "pathfinder_poseidon";

/// Main entry point of the committer CLI.
fn main() {
    let real_start = Instant::now();

    // Read input from file
    let args: Vec<String> = env::args().collect();
    let input_file_path = Path::new(&args[1]);
    let output_file_path = Path::new(&args[2]);
    assert!(
        input_file_path.is_absolute() && output_file_path.is_absolute(),
        "Given paths must be absolute"
    );
    let input_file_name = input_file_path.file_name().unwrap().to_str().unwrap();
    let file: &[u8] = &std::fs::read(input_file_path).unwrap();

    let num_iterations: usize = 10000;
    let random_numbers: Vec<[u8; 32]> = file
        .chunks_exact(32)
        .map(|x| {
            [
                x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8], x[9], x[10], x[11], x[12],
                x[13], x[14], x[15], x[16], x[17], x[18], x[19], x[20], x[21], x[22], x[23], x[24],
                x[25], x[26], x[27], x[28], x[29], x[30], x[31],
            ]
        })
        .collect();
    let output: [u8; 32];

    let now = Instant::now();
    // Code block to measure.
    if input_file_name.ends_with(PATHFINDER_PEDERSEN) {
        let random_felts: Vec<Felt> = random_numbers
            .iter()
            .map(|x| Felt::from_be_bytes(*x).expect("Overflow should not happen."))
            .collect();
        let mut result_pathfinder = random_felts[0];
        for random_felt in random_felts.iter() {
            result_pathfinder =
                pathfinder_crypto::hash::pedersen_hash(*random_felt, result_pathfinder);
        }
        output = *result_pathfinder.as_be_bytes();
    } else if input_file_name.ends_with(LC_PEDERSEN) {
        let random_field_elements: Vec<FieldElement> = random_numbers
            .iter()
            .map(|x| FieldElement::from_bytes_be(x).expect("Overflow should not happen."))
            .collect();

        let mut result_lc = random_field_elements[0];
        for random_field_element in random_field_elements.iter() {
            result_lc = starknet::core::crypto::pedersen_hash(random_field_element, &result_lc);
        }
        output = result_lc.to_bytes_be();
    } else if input_file_name.ends_with(STARKNET_PEDERSEN) {
        let random_sn_felts: Vec<StarknetFelt> = random_numbers
            .iter()
            .map(StarknetFelt::from_bytes_be)
            .collect();
        let mut result_sn = random_sn_felts[0];
        for random_sn_felt in random_sn_felts.iter() {
            result_sn = Pedersen::hash(random_sn_felt, &result_sn);
        }
        output = result_sn.to_bytes_be();
    } else if input_file_name.ends_with(PATHFINDER_POSEIDON) {
        let random_mont_felts: Vec<MontFelt> = random_numbers
            .iter()
            .map(|x| MontFelt::from_be_bytes(*x))
            .collect();
        let mut result_pathfinder_poseidon = random_mont_felts[0];
        for random_mont_felt in random_mont_felts.iter() {
            result_pathfinder_poseidon = pathfinder_crypto::hash::poseidon_hash(
                *random_mont_felt,
                result_pathfinder_poseidon,
            );
        }
        output = result_pathfinder_poseidon.to_be_bytes();
    } else {
        panic!("Invalid input file name");
    }
    // End of code block to measure.
    let elapased_time = now.elapsed();

    // Sanity check.
    if input_file_name.ends_with("pedersen") {
        assert_eq!(
            Felt::from_be_bytes(output).unwrap().to_hex_str(),
            "0x577d066959268ce144d1e4cd42dbb004883997302d6b056178028d515f0a61b"
        );
    } else {
        assert_ne!(
            Felt::from_be_bytes(output).unwrap().to_hex_str(),
            "0x577d066959268ce144d1e4cd42dbb004883997302d6b056178028d515f0a61b"
        );
    }
    // Print measurement.
    println!(
        "Time for {num_iterations} hashes: {:?}, for a single hash: {:?}",
        elapased_time,
        elapased_time.checked_div(num_iterations.try_into().unwrap()),
    );

    // Output to file.
    std::fs::write(output_file_path, output).expect("Failed to write output");
    println!("Total time: {:?}", real_start.elapsed());
}
