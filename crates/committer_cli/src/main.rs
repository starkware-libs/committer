use clap::{Args, Parser, Subcommand};
use std::path::Path;

/// Committer CLI.
#[derive(Debug, Parser)]
#[clap(name = "committer-cli", version)]
pub struct CommitterCliArgs {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Given previous state tree skeleton and a state diff, computes the new commitment.
    Commit {
        /// File path to input.
        #[clap(long, short = 'i', default_value = "stdin")]
        input_path: String,

        /// File path to output.
        #[clap(long, short = 'o', default_value = "stdout")]
        output_path: String,

        /// The version of the class hash, as a string (before conversion to raw bytes).
        #[clap(long)]
        class_hash_version: String,

        /// The version of the contract state hash, as a string (before conversion to raw bytes).
        #[clap(long)]
        contract_state_hash_version: String,
    },
}

#[derive(Debug, Args)]
struct GlobalOpts {}

/// Main entry point of the committer CLI.
fn main() {
    let args = CommitterCliArgs::parse();

    match args.command {
        Command::Commit {
            input_path,
            output_path,
            class_hash_version: _,
            contract_state_hash_version: _,
        } => {
            let input_file_name = Path::new(&input_path);
            let output_file_name = Path::new(&output_path);
            assert!(
                input_file_name.is_absolute() && output_file_name.is_absolute(),
                "Given paths must be absolute."
            );

            // Business logic to be implemented here.
            let output = std::fs::read(input_file_name).unwrap();

            // Output to file.
            std::fs::write(output_file_name, output).expect("Failed to write output");
        }
    }
}
