use std::ops;

use crate::{patricia_merkle_tree::types::EdgePathLength, types::Felt};

#[allow(dead_code)]
pub(crate) struct HashInputPair(pub Felt, pub Felt);

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub(crate) struct HashOutput(pub Felt);

#[allow(dead_code)]
impl HashOutput {
    pub(crate) const ZERO: HashOutput = HashOutput(Felt::ZERO);
}

pub(crate) trait HashFunction {
    /// Computes the hash of given input.
    fn compute_hash(i: HashInputPair) -> HashOutput;
}

/// Field addition, Never overflows/underflows. Used for computing the hash of Edge nodes.
impl ops::Add<EdgePathLength> for HashOutput {
    type Output = HashOutput;

    fn add(self, rhs: EdgePathLength) -> Self::Output {
        Self(self.0 + Felt::from(rhs.0))
    }
}
