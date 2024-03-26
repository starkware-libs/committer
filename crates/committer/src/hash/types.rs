use crate::types::Felt;

pub(crate) struct HashInputPair(pub Felt, pub Felt);

pub(crate) struct HashOutput(pub Felt);

#[allow(dead_code)]
impl HashOutput {
    pub(crate) const ZERO: HashOutput = HashOutput(Felt::ZERO);
}

pub(crate) trait HashFunction {
    /// Computes the hash of given input.
    async fn compute_hash(i: HashInputPair) -> HashOutput;
}
