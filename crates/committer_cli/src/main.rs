use clap::{Args, Parser, Subcommand};
use std::{collections::HashSet, path::Path};

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

        /// The version of the class hash.
        #[clap(long)]
        class_hash_version: u8,

        /// The version of the contract state hash.
        #[clap(long)]
        contract_state_hash_version: u8,

        #[clap(flatten)]
        leaf_prefixes: LeafPrefixes,

        #[clap(flatten)]
        node_prefixes: NodePrefixes,
    },
}

#[derive(Debug, Args)]
struct GlobalOpts {}

#[derive(Debug, Args)]
struct LeafPrefixes {
    /// Prefix for storage leaf.
    #[clap(long)]
    storage: u8,

    /// Prefix for contract state leaf.
    #[clap(long)]
    contract_state: u8,

    /// Prefix for class leaf.
    #[clap(long)]
    class: u8,
}

#[derive(Debug, Args)]
struct NodePrefixes {
    /// Prefix for edge node.
    #[clap(long)]
    edge: u8,

    /// Prefix for sibling node.
    #[clap(long)]
    sibling: u8,

    /// Prefix for binary node.
    #[clap(long)]
    binary: u8,

    /// Prefix for empty node.
    #[clap(long)]
    empty: u8,
}

/// Main entry point of the committer CLI.
fn main() {
    let args = CommitterCliArgs::parse();

    match args.command {
        Command::Commit {
            input_path,
            output_path,
            class_hash_version: _,
            contract_state_hash_version: _,
            leaf_prefixes,
            node_prefixes,
        } => {
            // Input verification: prefixes must be unique.
            let expected_unique_nodes = 4;
            let prefixes = [
                node_prefixes.edge,
                node_prefixes.sibling,
                node_prefixes.binary,
                node_prefixes.empty,
            ];
            if HashSet::from(prefixes).len() != expected_unique_nodes {
                panic!("Node prefixes must be unique, got {prefixes:?}.");
            }
            let expected_unique_leaves = 3;
            let leaf_prefixes = [
                leaf_prefixes.storage,
                leaf_prefixes.contract_state,
                leaf_prefixes.class,
            ];
            if HashSet::from(leaf_prefixes).len() != expected_unique_leaves {
                panic!("Leaf prefixes must be unique, got {leaf_prefixes:?}.");
            }

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
