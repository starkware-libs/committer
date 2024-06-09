use std::collections::HashMap;

use crate::felt::Felt;

use crate::hash::hash_trait::HashOutput;
use crate::patricia_merkle_tree::errors::TypesError;
use crate::patricia_merkle_tree::filled_tree::tree::{FilledTree, FilledTreeImpl};
use crate::patricia_merkle_tree::node_data::inner_node::{EdgePathLength, PathToBottom};
use crate::patricia_merkle_tree::node_data::leaf::SkeletonLeaf;
use crate::patricia_merkle_tree::original_skeleton_tree::tree::{
    OriginalSkeletonTree, OriginalSkeletonTreeImpl,
};
use crate::patricia_merkle_tree::types::NodeIndex;
use crate::patricia_merkle_tree::updated_skeleton_tree::tree::{
    UpdatedSkeletonTree, UpdatedSkeletonTreeImpl,
};
use crate::storage::map_storage::MapStorage;
use ethnum::U256;
use rand::rngs::ThreadRng;
use rand::Rng;
use rstest::{fixture, rstest};

use crate::patricia_merkle_tree::node_data::leaf::{LeafDataImpl, LeafModifications};

use crate::patricia_merkle_tree::updated_skeleton_tree::hash_function::TreeHashFunctionImpl;

impl TryFrom<&U256> for Felt {
    type Error = TypesError<U256>;
    fn try_from(value: &U256) -> Result<Self, Self::Error> {
        if *value > U256::from(&Felt::MAX) {
            return Err(TypesError::ConversionError {
                from: *value,
                to: "Felt",
                reason: "value is bigger than felt::max",
            });
        }
        Ok(Self::from_bytes_be(&value.to_be_bytes()))
    }
}

impl From<u8> for SkeletonLeaf {
    fn from(value: u8) -> Self {
        Self::from(Felt::from(value))
    }
}

impl From<&str> for PathToBottom {
    fn from(value: &str) -> Self {
        Self::new(
            U256::from_str_radix(value, 2)
                .expect("Invalid binary string")
                .into(),
            EdgePathLength::new(
                (value.len() - if value.starts_with('+') { 1 } else { 0 })
                    .try_into()
                    .expect("String is too large"),
            )
            .expect("Invalid length"),
        )
        .expect("Illegal PathToBottom")
    }
}

#[fixture]
pub(crate) fn random() -> ThreadRng {
    rand::thread_rng()
}

#[cfg(test)]
use crate::patricia_merkle_tree::types::SubTreeHeight;

#[cfg(test)]
impl NodeIndex {
    /// Assumes self represents an index in a smaller tree height. Returns a node index represents
    /// the same index in the starknet state tree as if the smaller tree was 'planted' at the lowest
    /// leftmost node from the root.
    pub(crate) fn from_subtree_index(subtree_index: Self, subtree_height: SubTreeHeight) -> Self {
        let height_diff = SubTreeHeight::ACTUAL_HEIGHT.0 - subtree_height.0;
        let offset = (NodeIndex::ROOT << height_diff) - 1.into();
        subtree_index + (offset << (subtree_index.bit_length() - 1))
    }
}

#[cfg(test)]
pub(crate) fn small_tree_index_to_full(index: U256, height: SubTreeHeight) -> NodeIndex {
    NodeIndex::from_subtree_index(NodeIndex::new(index), height)
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

#[cfg(test)]
pub(crate) fn as_fully_indexed(
    subtree_height: u8,
    indices: impl Iterator<Item = U256>,
) -> Vec<NodeIndex> {
    indices
        .map(|index| small_tree_index_to_full(index, SubTreeHeight::new(subtree_height)))
        .collect()
}

pub async fn rust_single_tree_flow_test(
    leaf_modifications: LeafModifications<LeafDataImpl>,
    storage: MapStorage,
    root_hash: HashOutput,
) -> String {
    // Move from leaf number to actual index.
    let leaf_modifications = leaf_modifications
        .into_iter()
        .map(|(k, v)| (NodeIndex::FIRST_LEAF + k, v))
        .collect::<HashMap<NodeIndex, LeafDataImpl>>();
    let mut sorted_leaf_indices: Vec<NodeIndex> = leaf_modifications.keys().copied().collect();
    sorted_leaf_indices.sort();

    // Build the original tree.
    let mut original_skeleton: OriginalSkeletonTreeImpl =
        OriginalSkeletonTree::create(&storage, &sorted_leaf_indices, root_hash)
            .expect("Failed to create the original skeleton tree");

    // Update the tree with the new data.
    let updated_skeleton: UpdatedSkeletonTreeImpl =
        UpdatedSkeletonTree::create_from_modifications_data(
            &mut original_skeleton,
            &leaf_modifications,
        )
        .expect("Failed to create the updated skeleton tree");

    // Compute the hash.
    let filled_tree: FilledTreeImpl =
        FilledTreeImpl::create::<TreeHashFunctionImpl>(updated_skeleton, leaf_modifications)
            .await
            .expect("Failed to create the filled tree");
    let hash_result = filled_tree
        .get_root_hash()
        .expect("Failed to get the root hash");

    // Serialize the hash result.
    hash_result.0.to_hex()

    // TODO(Aner, 9/6/24): Serlialize the storage modifications.
}
