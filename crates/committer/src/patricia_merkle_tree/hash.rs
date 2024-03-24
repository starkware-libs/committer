use pathfinder_crypto::Felt;

pub(crate) struct HashInput(pub Felt, pub Felt, pub Felt);

pub(crate) struct HashOutput(pub Felt);

pub(crate) trait HashFunction {
    /// Computes the hash of given input.
    fn compute_hash(i: HashInput) -> HashOutput;
}
