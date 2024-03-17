use std::env;
use std::path::Path;

/// Gets a binary input and an output file from the command line.
/// Reads the input file, adds 1 to each byte, and writes the result to the output file.
fn main() {
    // Open the input file
    let args: Vec<String> = env::args().collect();
    let input_file_name = Path::new(&args[1]);
    let output_file_name = Path::new(&args[2]);
    assert!(
        input_file_name.is_absolute() && output_file_name.is_absolute(),
        "Given paths must be absolute"
    );
    let mut bytes = std::fs::read(input_file_name).unwrap();

    // Add 1 to each byte
    for byte in &mut bytes {
        *byte = byte.wrapping_add(1);
    }
    std::fs::write(output_file_name, bytes).expect("Failed to write output");
}
