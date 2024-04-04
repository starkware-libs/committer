mod args;

use args::InputArgs;
use clap::{command, value_parser, Arg, ArgMatches};
use std::{collections::HashSet, path::Path};

use crate::args::{LeafPrefixes, NodePrefixes, StoragePrefix};

/// Main entry point of the committer CLI.
fn main() {
    let args = args();
    match args {
        InputArgs::CurrentArgs {
            input_path,
            output_path,
            class_hash_version: _,
            contract_state_hash_version: _,
            leaf_prefixes: _,
            node_prefixes: _,
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
fn create_args() -> ArgMatches {
    command!()
        .about("Given previous state tree skeleton and a state diff, computes the new commitment.")
        .arg(
            Arg::new("class_hash_version")
                .long("class-hash-version")
                .required(true)
                .help("An input when hashing a new leaf at the contract class patricia tree.")
                .value_name("Integer")
                .value_parser(value_parser!(u8)),
        )
        .arg(
            Arg::new("contract_state_hash_version")
                .long("contract-state-hash-version")
                .required(true)
                .help("An input when hashing a leaf at the contract instance patricia tree.")
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
                .help("The prefix used to serialize a sibling node."),
        )
        .arg(
            Arg::new("empty_node_prefix")
                .long("empty-node-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serialize an empty node."),
        )
        .arg(
            Arg::new("edge_node_prefix")
                .long("edge-node-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serialize an edge node."),
        )
        .arg(
            Arg::new("binary_node_prefix")
                .long("binary-node-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serialize a binary node."),
        )
        .arg(
            Arg::new("storage_leaf_prefix")
                .long("storage-leaf-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serialize a storage leaf."),
        )
        .arg(
            Arg::new("contract_state_leaf_prefix")
                .long("contract-state-leaf-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serialize a contract state leaf."),
        )
        .arg(
            Arg::new("class_leaf_prefix")
                .long("class-leaf-prefix")
                .required(true)
                .value_name("Integer")
                .value_parser(value_parser!(u8))
                .help("The prefix used to serialize a compiled class hash leaf."),
        )
        .get_matches()
}

fn args() -> InputArgs {
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

    let edge = StoragePrefix(*args.get_one::<u8>("edge_node_prefix").expect(error_message));
    let empty = StoragePrefix(
        *args
            .get_one::<u8>("empty_node_prefix")
            .expect(error_message),
    );
    let sibling = StoragePrefix(
        *args
            .get_one::<u8>("sibling_node_prefix")
            .expect(error_message),
    );
    let binary = StoragePrefix(
        *args
            .get_one::<u8>("binary_node_prefix")
            .expect(error_message),
    );

    // Make sure the prefixes are unique.
    let expected_unique_nodes = 4;
    assert_eq!(
        HashSet::from([binary.0, sibling.0, empty.0, edge.0]).len(),
        expected_unique_nodes
    );

    let storage = StoragePrefix(
        *args
            .get_one::<u8>("storage_leaf_prefix")
            .expect(error_message),
    );
    let contract_state = StoragePrefix(
        *args
            .get_one::<u8>("contract_state_leaf_prefix")
            .expect(error_message),
    );
    let class = StoragePrefix(
        *args
            .get_one::<u8>("class_leaf_prefix")
            .expect(error_message),
    );

    // Make sure the prefixes are unique.
    let expected_unique_leaves = 3;
    assert_eq!(
        HashSet::from([storage.0, contract_state.0, class.0]).len(),
        expected_unique_leaves
    );

    InputArgs::CurrentArgs {
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
