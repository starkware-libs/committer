use std::collections::HashMap;

use crate::felt::Felt;
use crate::hash::hash_trait::HashOutput;
use crate::storage::map_storage::MapStorage;
use ethnum::U256;
use rand::rngs::ThreadRng;
use rand::Rng;
use rstest::{fixture, rstest};

use crate::patricia_merkle_tree::node_data::inner_node::{EdgePathLength, PathToBottom};
use crate::patricia_merkle_tree::node_data::leaf::SkeletonLeaf;

use crate::patricia_merkle_tree::filled_tree::tree::{FilledTree, FilledTreeImpl};
use crate::patricia_merkle_tree::original_skeleton_tree::tree::{
    OriginalSkeletonTree, OriginalSkeletonTreeImpl,
};
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::{
    UpdatedSkeletonTree, UpdatedSkeletonTreeImpl,
};

use crate::patricia_merkle_tree::node_data::leaf::{LeafDataImpl, LeafModifications};

use super::node_data::leaf::LeafData;
use super::updated_skeleton_tree::hash_function::TreeHashFunctionImpl;

// use super::original_skeleton_tree::tree::{OriginalSkeletonTree, OriginalSkeletonTreeImpl};

impl From<u8> for SkeletonLeaf {
    fn from(value: u8) -> Self {
        Self::from(Felt::from(value))
    }
}

impl From<&str> for PathToBottom {
    fn from(value: &str) -> Self {
        Self {
            path: U256::from_str_radix(value, 2)
                .expect("Invalid binary string")
                .into(),
            length: EdgePathLength::new(
                (value.len() - if value.starts_with('+') { 1 } else { 0 })
                    .try_into()
                    .expect("String is too large"),
            )
            .expect("Invalid length"),
        }
    }
}

#[fixture]
pub(crate) fn random() -> ThreadRng {
    rand::thread_rng()
}

/// Generates a random U256 number between low and high (exclusive).
/// Panics if low > high.
#[cfg(any(feature = "testing", test))]
pub fn get_random_u256<R: Rng>(rng: &mut R, low: U256, high: U256) -> U256 {
    assert!(low < high);
    let high_of_low = low.high();
    let high_of_high = high.high();

    let delta = high - low;
    if delta <= u128::MAX {
        let delta = u128::try_from(delta).expect("Failed to convert delta to u128");
        return low + rng.gen_range(0..delta);
    }

    // Randomize the high 128 bits in the extracted range, and the low 128 bits in their entire
    // domain until the result is in range.
    // As high-low>u128::MAX, the expected number of samples until the loops breaks is bound from
    // above by 3 (as either:
    //  1. high_of_high > high_of_low + 1, and there is a 1/3 chance to get a valid result for high
    //  bits in (high_of_low, high_of_high).
    //  2. high_of_high == high_of_low + 1, and every possible low 128 bits value is valid either
    // when the high bits equal high_of_high, or when they equal high_of_low).
    let mut randomize = || {
        U256::from_words(
            rng.gen_range(*high_of_low..=*high_of_high),
            rng.gen_range(0..=u128::MAX),
        )
    };
    let mut result = randomize();
    while result < low || result >= high {
        result = randomize();
    }
    result
}

#[rstest]
#[should_panic]
#[case(U256::ZERO, U256::ZERO)]
#[case(U256::ZERO, U256::ONE)]
#[case(U256::ONE, U256::ONE << 128)]
#[case((U256::ONE<<128)-U256::ONE, U256::ONE << 128)]
#[case(U256::ONE<<128, (U256::ONE << 128)+U256::ONE)]
fn test_get_random_u256(mut random: ThreadRng, #[case] low: U256, #[case] high: U256) {
    let r = get_random_u256(&mut random, low, high);
    assert!(low <= r && r < high);
}

fn get_binary_modifications(
    leaf_modifications: &LeafModifications<LeafDataImpl>,
) -> LeafModifications<SkeletonLeaf> {
    let mut binary_modifications = LeafModifications::new();
    for (index, data) in leaf_modifications.iter() {
        binary_modifications.insert(
            *index,
            match data.is_empty() {
                true => SkeletonLeaf::Zero,
                false => SkeletonLeaf::NonZero,
            },
        );
    }
    binary_modifications
}

pub async fn rust_single_tree_flow_test(
    leaf_modifications: LeafModifications<LeafDataImpl>,
    storage: MapStorage,
    root_hash: HashOutput,
) -> String {
    println!(
        "leaf_modifications (in leaf index): {:?}",
        leaf_modifications
    );
    //Move from leaf number to actual index
    let leaf_modifications = leaf_modifications
        .into_iter()
        .map(|(k, v)| (NodeIndex::FIRST_LEAF + k, v))
        .collect::<HashMap<NodeIndex, LeafDataImpl>>();
    println!("leaf_modifications: {:?}", leaf_modifications);
    // let mut sorted_leaf_indices: Vec<NodeIndex> = leaf_modifications
    //     .keys()
    //     .map(|index| (NodeIndex::FIRST_LEAF + *index))
    //     // .map(|address| NodeIndex::from_contract_address(address, &TreeHeight::MAX))
    //     .collect();
    // sorted_leaf_indices.sort();
    // Get the tree data from the input.
    let leaf_modifications_binary = get_binary_modifications(&leaf_modifications);
    println!(
        "leaf_modifications_binary: {:?} ",
        leaf_modifications_binary
    );
    let mut sorted_leaf_indices: Vec<NodeIndex> = leaf_modifications.keys().copied().collect();
    sorted_leaf_indices.sort();

    // Build the original tree.
    let mut original_skeleton: OriginalSkeletonTreeImpl =
        OriginalSkeletonTree::create(&storage, &sorted_leaf_indices, root_hash)
            .expect("Failed to create the original skeleton tree");

    println!("Constructed original_skeleton: {:?}", original_skeleton);

    println!("Starting to create the updated skeleton tree");

    // [Optional] Compute the intermediate hash for sanity check.

    // Update the tree with the new data.
    let updated_skeleton: UpdatedSkeletonTreeImpl =
        UpdatedSkeletonTree::create(&mut original_skeleton, &leaf_modifications_binary)
            .expect("Failed to create the updated skeleton tree");

    println!(
        "Constructed updated_skeleton: {:?}",
        updated_skeleton.skeleton_tree
    );

    print!("TADA");

    println!("Starting to create the filled tree");
    // Compute the hash.
    print!("leaf_modifications: {:?}", leaf_modifications);
    let filled_tree: FilledTreeImpl =
        FilledTreeImpl::create::<TreeHashFunctionImpl>(updated_skeleton, leaf_modifications)
            .await
            .expect("Failed to create the filled tree");
    let hash_result = filled_tree
        .get_root_hash()
        .expect("Failed to get the root hash");

    // Serialize the hash result (including intermediate hash??).

    hash_result.0.to_hex()
}
