use crate::felt::Felt;
use ethnum::U256;
use rand::rngs::ThreadRng;
use rand::Rng;
use rstest::{fixture, rstest};

use crate::patricia_merkle_tree::node_data::inner_node::{EdgePathLength, PathToBottom};
use crate::patricia_merkle_tree::node_data::leaf::SkeletonLeaf;

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
            length: EdgePathLength(
                (value.len() - if value.starts_with('+') { 1 } else { 0 })
                    .try_into()
                    .expect("String is too large"),
            ),
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
