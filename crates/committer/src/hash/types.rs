use crate::types::CommitterFelt;

pub(crate) struct HashInput(pub CommitterFelt, pub CommitterFelt);

pub(crate) struct HashOutput(pub CommitterFelt);

pub(crate) trait HashFunction {
    /// Computes the hash of given input.
    async fn compute_hash(i: HashInput) -> HashOutput;
}
