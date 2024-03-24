use pathfinder_crypto::Felt;

use super::hash::HashFunction;

pub(crate) struct NodeIndex(pub Felt);

// TODO(Amos, 26/3/2024): Define trait's methods.
pub(crate) trait Node<H: HashFunction> {}
