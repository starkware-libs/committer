use starknet_types_core::felt::FromStrError;

use crate::felt::Felt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HashOutput(pub Felt);

impl HashOutput {
    #[allow(dead_code)]
    pub(crate) const ZERO: HashOutput = HashOutput(Felt::ZERO);
    pub(crate) const ROOT_OF_EMPTY_TREE: HashOutput = Self::ZERO;

    pub(crate) fn from_hex(hex_string: &str) -> Result<Self, FromStrError> {
        Ok(Self(Felt::from_hex(hex_string)?))
    }
}
