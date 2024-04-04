use clap::{command, value_parser, Arg, ArgMatches};
use std::{collections::HashSet, env, path::Path};

/// Main entry point of the committer CLI.
fn main() {
    let args = args();
    let input_file_name = Path::new(&args.input_path);
    let output_file_name = Path::new(&args.output_path);
    assert!(
        input_file_name.is_absolute() && output_file_name.is_absolute(),
        "Given paths must be absolute."
    );

    // Business logic to be implemented here.
    let output = std::fs::read(input_file_name).unwrap();

    // Output to file.
    std::fs::write(output_file_name, output).expect("Failed to write output");
}
fn create_args() -> ArgMatches {
    command!()
        // TODO(Nimrod, 20/4/2024): Write some explanation below.
        .about("Some explanation about the committer.")
        .arg(
            Arg::new("class_hash_version")
                .long("class-hash-version")
                .required(true)
                .help("an input when hashing a new leaf at the contract class merkle tree.")
                .value_name("Integer")
                .value_parser(value_parser!(u8)),
        )
        .arg(
            Arg::new("contract_state_hash_version")
                .long("contract-state-hash-version")
                .required(true)
                .help("an input when hashing a leaf at the contract instance merkle tree.")
                .value_name("Integer")
                .value_parser(value_parser!(u8)),
        )
        .arg(
            Arg::new("input_file")
                .value_name("FILE")
                .long("input-file")
                .default_value("stdin")
                .short('i'),
        )
        .arg(
            Arg::new("output_file")
                .value_name("FILE")
                .long("output-file")
                .default_value("stdout")
                .short('o'),
        )
        .arg(
            Arg::new("sibling_node_prefix")
                .long("sibling-node-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serailize a sibling node."),
        )
        .arg(
            Arg::new("empty_node_prefix")
                .long("empty-node-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serailize an empty node."),
        )
        .arg(
            Arg::new("edge_node_prefix")
                .long("edge-node-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serailize an edge node."),
        )
        .arg(
            Arg::new("binary_node_prefix")
                .long("binary-node-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serailize a binary node."),
        )
        .arg(
            Arg::new("storage_leaf_prefix")
                .long("storage-leaf-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serailize a storage leaf."),
        )
        .arg(
            Arg::new("contract_state_leaf_prefix")
                .long("contract-state-leaf-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serailize a contract state leaf."),
        )
        .arg(
            Arg::new("class_leaf_prefix")
                .long("class-leaf-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serailize a class leaf."),
        )
        .get_matches()
}

fn args() -> Args {
    let args = create_args();

    let error_message = "Should not fail as this argument is required or has a default value.";
    let input_path = args
        .get_one::<String>("input_file")
        .expect(error_message)
        .to_string();
    let output_path = args
        .get_one::<String>("output_file")
        .expect(error_message)
        .to_string();
    let class_hash_version = *args
        .get_one::<u8>("class_hash_version")
        .expect(error_message);
    let contract_state_hash_version = *args
        .get_one::<u8>("contract_state_hash_version")
        .expect(error_message);

    let edge = *args.get_one::<u8>("edge_node_prefix").expect(error_message);
    let empty = *args
        .get_one::<u8>("empty_node_prefix")
        .expect(error_message);
    let sibling = *args
        .get_one::<u8>("sibling_node_prefix")
        .expect(error_message);
    let binary = *args
        .get_one::<u8>("binary_node_prefix")
        .expect(error_message);

    // Make sure the prefixes are unique.
    let expected_unique_nodes = 4;
    assert_eq!(
        HashSet::from([binary, sibling, empty, edge]).len(),
        expected_unique_nodes
    );

    let storage = *args
        .get_one::<u8>("storage_leaf_prefix")
        .expect(error_message);
    let contract_state = *args
        .get_one::<u8>("contract_state_leaf_prefix")
        .expect(error_message);
    let class = *args
        .get_one::<u8>("class_leaf_prefix")
        .expect(error_message);

    // Make sure the prefixes are unique.
    let expected_unique_leaves = 3;
    assert_eq!(
        HashSet::from([storage, contract_state, class]).len(),
        expected_unique_leaves
    );

    Args {
        input_path,
        output_path,
        class_hash_version,
        contract_state_hash_version,
        node_prefixes: NodePrefixes {
            edge,
            sibling,
            binary,
            empty,
        },
        leaf_prefixes: LeafPrefixes {
            storage,
            contract_state,
            class,
        },
    }
}
#[allow(dead_code)]
#[derive(Debug)]
/// Holds all the information needed for the committer.
struct Args {
    input_path: String,
    output_path: String,
    class_hash_version: u8,
    contract_state_hash_version: u8,
    leaf_prefixes: LeafPrefixes,
    node_prefixes: NodePrefixes,
}
#[allow(dead_code)]
#[derive(Debug)]
struct LeafPrefixes {
    storage: u8,
    contract_state: u8,
    class: u8,
}
#[allow(dead_code)]
#[derive(Debug)]
struct NodePrefixes {
    edge: u8,
    sibling: u8,
    binary: u8,
    empty: u8,
}
