use crate::felt::Felt;

pub struct HashInputPair(pub Felt, pub Felt);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HashOutput(pub Felt);

#[allow(dead_code)]
impl HashOutput {
    pub(crate) const ZERO: HashOutput = HashOutput(Felt::ZERO);
}

pub trait HashFunction {
    /// Computes the hash of given input.
    fn compute_hash(i: HashInputPair) -> HashOutput;
}
