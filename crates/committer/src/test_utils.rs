use pathfinder_crypto::Felt;
use rand::Rng;
use std::time::Duration;

pub fn mean(data: &[Duration]) -> Duration {
    data.iter()
        .sum::<Duration>()
        .checked_div(data.len().try_into().unwrap())
        .unwrap()
}

#[allow(clippy::as_conversions)]
pub fn std_deviation(data: &[Duration]) -> Duration {
    let mean = mean(data).as_secs_f32();
    let mut variance = data
        .iter()
        .map(|x| {
            let diff = (*x).as_secs_f32() - mean;
            diff * diff
        })
        .sum::<f32>();
    variance /= data.len() as f32;
    Duration::from_secs_f32(variance.sqrt())
}

pub fn random_felt() -> Felt {
    let mut buf: [u8; 32] = rand::thread_rng().gen();
    buf[0] &= 0x07; // clear the 5 most significant bits
    Felt::from_be_bytes(buf).expect("Overflow ;(")
}
