use crate::types::CommitterFelt;

pub(crate) struct HashInputPair(pub CommitterFelt, pub CommitterFelt);

pub(crate) struct HashOutput(pub CommitterFelt);

#[allow(dead_code)]
impl HashOutput {
    pub(crate) const ZERO: HashOutput = HashOutput(CommitterFelt::ZERO);
}

pub(crate) trait HashFunction {
    /// Computes the hash of given input.
    async fn compute_hash(i: HashInputPair) -> HashOutput;
}
