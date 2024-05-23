use crate::patricia_merkle_tree::utils::random_structs::get_random_u256;
use ethnum::U256;
use rstest::rstest;

use crate::patricia_merkle_tree::node_data::inner_node::{EdgePathLength, PathToBottom};

impl From<&str> for PathToBottom {
    fn from(value: &str) -> Self {
        Self {
            path: U256::from_str_radix(value, 2)
                .expect("Invalid binary string")
                .into(),
            length: EdgePathLength(
                (value.len() - if value.starts_with('+') { 1 } else { 0 })
                    .try_into()
                    .expect("String is too large"),
            ),
        }
    }
}

#[rstest]
#[should_panic]
#[case(U256::ZERO, U256::ZERO)]
#[case(U256::ZERO, U256::ONE)]
#[case(U256::ONE, U256::ONE << 128)]
#[case((U256::ONE<<128)-U256::ONE, U256::ONE << 128)]
#[case(U256::ONE<<128, (U256::ONE << 128)+U256::ONE)]
fn test_get_random_u256(#[case] low: U256, #[case] high: U256) {
    let r = get_random_u256(low, high);
    assert!(low <= r && r < high);
}
