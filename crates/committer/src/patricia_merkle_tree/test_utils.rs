use ethnum::U256;
use rand::Rng;
use rstest::rstest;

/// Generates a random U256 number between low and high (exclusive).
/// Panics if low > high.
pub(crate) fn get_random_u256(low: U256, high: U256) -> U256 {
    let high_of_low = low.high();
    let high_of_high = high.high();
    assert!(high_of_low <= high_of_high);

    let delta = high - low;
    if delta <= u128::MAX {
        let delta = u128::try_from(delta).unwrap();
        return low + rand::thread_rng().gen_range(0..delta);
    }

    // Randomize the high 128 bits in the extracted range, and the low 128 bits in their entire
    // domain until the result is in range.
    // As high-low>u128::MAX, the expected number of samples until the loops breaks is bound from
    // above by 3 (as either high_of_high > high_of_low + 1, and there is a 1/3 chance to get a
    // valid result for high bits in (high_of_low, high_of_high) or high_of_high == high_of_low + 1)
    // and every possible low 128 bits value is valid when either the high bits equal high_of_high,
    // or high_of_low).
    let randomize = || {
        U256::from_words(
            rand::thread_rng().gen_range(*high_of_low..=*high_of_high),
            rand::thread_rng().gen_range(0..=u128::MAX),
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
fn test_get_random_u256(#[case] low: U256, #[case] high: U256) {
    let r = get_random_u256(low, high);
    assert!(low <= r && r < high);
}
