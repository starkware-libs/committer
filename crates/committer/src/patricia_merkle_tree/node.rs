use pathfinder_crypto::Felt;

use super::hash::HashFunction;

pub(crate) struct NodeIndex(pub Felt);

pub(crate) trait Node<H: HashFunction> {}
