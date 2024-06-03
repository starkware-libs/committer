use crate::felt::Felt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HashOutput(pub Felt);

impl HashOutput {
    #[allow(dead_code)]
    pub(crate) const ZERO: HashOutput = HashOutput(Felt::ZERO);
    pub(crate) const EMPTY_NODE: HashOutput = Self::ZERO;
}
