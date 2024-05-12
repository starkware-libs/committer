use std::cmp::min;

use ethnum::U256;
use rand::Rng;
use rstest::rstest;

/// Generates a random U256 number between low and high (inclusive).
/// Panics if low > high.
pub(crate) fn get_random_u256(low: U256, high: U256) -> U256 {
    let (mut high_of_low, low_of_low) = low.into_words();
    let (mut high_of_high, low_of_high) = high.into_words();
    assert!(high_of_low <= high_of_high);

    if high_of_high == high_of_low {
        return U256::from_words(
            high_of_low,
            rand::thread_rng().gen_range(low_of_low..=low_of_high),
        );
    }

    let upper_bound = u128::try_from(min(U256::from(u128::MAX), high - low)).unwrap();
    let new_low: u128 = rand::thread_rng().gen_range(0..upper_bound);
    if new_low < low_of_low {
        high_of_low += 1;
    }
    if new_low > low_of_high {
        high_of_high -= 1;
    }

    U256::from_words(
        rand::thread_rng().gen_range(high_of_low..=high_of_high),
        new_low,
    )
}

#[rstest]
#[case(U256::ZERO, U256::ZERO)]
#[case(U256::ZERO, U256::ONE)]
#[case(U256::ONE, U256::ONE << 128)]
#[case((U256::ONE<<128)-U256::ONE, U256::ONE << 128)]
#[case(U256::ONE<<128, (U256::ONE << 128)+U256::ONE)]
fn test_get_random_u256(#[case] low: U256, #[case] high: U256) {
    let r = get_random_u256(low, high);
    assert!(low <= r && r <= high);
}
