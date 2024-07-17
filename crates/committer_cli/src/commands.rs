use committer::block_committer::{
    commit::commit_block,
    input::{ConfigImpl, Input},
};
use tracing::info;

use crate::{
    filled_tree_output::filled_forest::SerializedForest,
    parse_input::read::{parse_input, write_to_file},
};

pub async fn parse_and_commit(input_string: &str, output_path: String) {
    let input = parse_input(input_string).expect("Failed to parse the given input.");
    info!(
        "Parsed commiter input successfully. Original Contracts Trie Root Hash: {:?}, 
    Original Classes Trie Root Hash: {:?}",
        input.contracts_trie_root_hash, input.classes_trie_root_hash,
    );
    commit(input, output_path).await;
}

pub async fn commit(input: Input<ConfigImpl>, output_path: String) {
    let serialized_filled_forest = SerializedForest(
        commit_block(input)
            .await
            .expect("Failed to commit the given block."),
    );
    let output = serialized_filled_forest.forest_to_output();
    write_to_file(&output_path, &output);
    info!(
        "Successfully committed given block. Updated Contracts Trie Root Hash: {:?}, 
    Updated Classes Trie Root Hash: {:?}",
        output.contract_storage_root_hash, output.compiled_class_root_hash,
    );
}
