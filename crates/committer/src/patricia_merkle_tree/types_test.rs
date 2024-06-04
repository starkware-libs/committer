use crate::block_committer::input::{ContractAddress, StarknetStorageKey};
use crate::felt::Felt;
use crate::patricia_merkle_tree::node_data::inner_node::{EdgePathLength, PathToBottom};
use crate::patricia_merkle_tree::test_utils::{get_random_u256, random};
use crate::patricia_merkle_tree::types::NodeIndex;
use rand::rngs::ThreadRng;

use ethnum::U256;
use rand::Rng;
use rstest::rstest;

#[rstest]
#[case(1, 1, 1, 3)]
#[case(1, 0, 2, 4)]
#[case(0xDAD, 0xFEE, 12, 0xDADFEE)]
#[case(0xDEAFBEE, 0xBFF, 16, 0xDEAFBEE0BFF)]
fn test_compute_bottom_index(
    #[case] node_index: u128,
    #[case] path: u128,
    #[case] length: u8,
    #[case] expected: u128,
) {
    let bottom_index = NodeIndex::compute_bottom_index(
        NodeIndex::from(node_index),
        &PathToBottom {
            path: path.into(),
            length: EdgePathLength::new(length).unwrap(),
        },
    );
    let expected = NodeIndex::from(expected);
    assert_eq!(bottom_index, expected);
}

#[rstest]
fn test_cast_to_node_index(
    #[values(0, 15, 0xDEADBEEF)] leaf_index: u128,
    #[values(true, false)] from_contract_address: bool,
) {
    let expected_node_index = NodeIndex::FIRST_LEAF + leaf_index.into();
    let actual = if from_contract_address {
        NodeIndex::from_contract_address(&ContractAddress(Felt::from(leaf_index)))
    } else {
        NodeIndex::from_starknet_storage_key(&StarknetStorageKey(Felt::from(leaf_index)))
    };
    assert_eq!(actual, expected_node_index);
}

#[rstest]
#[case(U256::new(1), U256::new(1), U256::new(1))]
#[case(U256::new(2), U256::new(5), U256::new(2))]
#[case(U256::new(5), U256([2,0]), U256([2,0]))]
#[case(U256::new(8), U256::new(10), U256::new(2))]
#[case(U256::new(9), U256::new(12), U256::new(1))]
#[case(U256::new(1), U256::new(2), U256::new(1))]
#[case(U256::new(2), U256::new(1), U256::new(1))]
#[case(
    U256::from_words(1<<121, 0),
    U256::from_words(1<<123, 0),
    U256::from_words(1<<121, 0)
)]
#[case(
    U256::from_words(6<<121, 12109832645278165874326859176438297),
    U256::from_words(7<<121, 34269583569287659876592876529763453),
    U256([3,0])
)]
fn test_get_lca(#[case] node_index: U256, #[case] other: U256, #[case] expected: U256) {
    let root_index = NodeIndex::new(node_index);
    let other_index = NodeIndex::new(other);
    let lca = root_index.get_lca(&other_index);
    let expected = NodeIndex::new(expected);
    assert_eq!(lca, expected);
}

#[rstest]
fn test_get_lca_big(mut random: ThreadRng) {
    let lca = NodeIndex::new(get_random_u256(
        &mut random,
        U256::ZERO,
        (NodeIndex::MAX >> 1).into(),
    ));

    let left_child = lca << 1;
    let right_child = left_child + 1.into();
    let mut random_extension = |index: NodeIndex| {
        let extension_bits = index.leading_zeros();
        let extension: u128 = random.gen_range(0..(1 << extension_bits));
        (index << extension_bits) + NodeIndex::new(U256::from(extension))
    };

    let left_descendant = random_extension(left_child);
    let right_descendant = random_extension(right_child);
    assert_eq!(left_descendant.get_lca(&right_descendant), lca);
}

#[rstest]
#[case(3, 3, 0, 0)]
#[case(2, 10, 2, 2)]
#[should_panic]
#[case(2, 3, 0, 0)]
#[should_panic]
#[case(2, 6, 0, 0)]
#[should_panic]
#[case(6, 2, 0, 0)]
fn test_get_path_to_descendant(
    #[case] root_index: u8,
    #[case] descendant: u8,
    #[case] expected_path: u8,
    #[case] expected_length: u8,
) {
    let root_index = NodeIndex::new(root_index.into());
    let descendant = NodeIndex::new(descendant.into());
    let path_to_bottom = root_index.get_path_to_descendant(descendant);
    assert_eq!(path_to_bottom.path, U256::from(expected_path).into());
    assert_eq!(
        path_to_bottom.length,
        EdgePathLength::new(expected_length).unwrap()
    );
}

#[rstest]
fn test_get_path_to_descendant_big() {
    let root_index = NodeIndex::new(U256::from(rand::thread_rng().gen::<u128>()));
    let max_bits = NodeIndex::BITS - 128;
    let extension: u128 = rand::thread_rng().gen_range(0..1 << max_bits);
    let extension_index = NodeIndex::new(U256::from(extension));

    let descendant = (root_index << extension_index.bit_length()) + extension_index;
    let path_to_bottom = root_index.get_path_to_descendant(descendant);
    assert_eq!(path_to_bottom.path, extension.into());
    assert_eq!(
        path_to_bottom.length,
        EdgePathLength::new(extension_index.bit_length()).unwrap()
    );
}

#[rstest]
fn test_nodeindex_to_felt_conversion() {
    let index = NodeIndex::MAX;
    assert!(Felt::try_from(index).is_err());
}
