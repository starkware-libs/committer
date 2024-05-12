use ethnum::U256;
use rand::Rng;
use rstest::rstest;

///Generates a random U256 number between low and high (inclusive).
pub(crate) fn get_random_u256(low: U256, high: U256) -> U256 {
    let (high_of_low, low_of_low) = low.into_words();
    let (high_of_high, low_of_high) = high.into_words();
    let new_high_of_low = if low_of_low > low_of_high {
        high_of_low + 1
    } else {
        high_of_low
    };

    let new_high: u128 = rand::thread_rng()
        .gen_range(new_high_of_low..=high_of_high)
        .into();
    if new_high > high_of_low {
        if new_high < high_of_high {
            U256::from_words(new_high, rand::thread_rng().gen())
        } else {
            U256::from_words(new_high, rand::thread_rng().gen_range(0..=low_of_high))
        }
    } else {
        U256::from_words(
            new_high,
            rand::thread_rng().gen_range(low_of_low..=low_of_high),
        )
    }
}

#[rstest]
#[case(U256::ZERO, U256::ZERO)]
#[case(U256::ZERO, U256::ONE)]
#[case((U256::ONE<<128)-U256::ONE, U256::ONE << 128)]
#[case(U256::ONE<<128, (U256::ONE << 128)+U256::ONE)]
fn test_get_random_u256(#[case] low: U256, #[case] high: U256) {
    let r = get_random_u256(low, high);
    assert!(low <= r && r <= high);
}
